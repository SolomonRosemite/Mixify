package createPlaylistSnapshotConfiguration

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/SolomonRosemite/Mixify/api/graphql/model"
	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/SolomonRosemite/Mixify/internal/utils/common"
	"github.com/SolomonRosemite/Mixify/internal/utils/mixify"
	"gorm.io/gorm"
)

type playlistGroup struct {
	Playlist      *models.PlaylistSnapshot
	InputID       *uint
	PlaylistOrder []*int
}

func CreatePlaylistSnapshotConfiguration(ctx *context.Context, input *model.NewPlaylistSnapshotConfiguration, db *gorm.DB) (*model.PlaylistSnapshotConfiguration, error) {
	playlists := []*models.PlaylistSnapshot{}
	playlistGroups := []*playlistGroup{}

	for _, p := range input.Playlists {
		var err error
		var playlistID uint
		if playlistID, err = common.StringToUint(p.PlaylistID, 32); err != nil {
			return nil, err
		}

		newP := models.PlaylistSnapshot{
			Model:             gorm.Model{ID: playlistID},
			Name:              &p.Name,
			SpotifyPlaylistID: p.SpotifyPlaylistID,
			Associations:      &[]*models.PlaylistAssociationSnapshot{},
		}

		playlists = append(playlists, &newP)
		playlistGroups = append(playlistGroups, &playlistGroup{Playlist: &newP, InputID: &playlistID, PlaylistOrder: p.PlaylistOrder})
	}

	for _, p := range input.Playlists {
		err := assignAssociations(p, &playlists)
		if err != nil {
			return nil, err
		}
	}

	// We create the graph to verify no circular dependencies exist
	mixify.CreateMixStackGraph(&playlists)

	// Since the playlist ids where only set to create the graph, once the graph is created we have to reset the ids
	// to zero. If not gorm will update the playlists instead.
	resetPlaylistIds(&playlists)

	// TODO: Replace test user id
	playlistSnapshotConfiguration := models.PlaylistConfigurationSnapshot{UserID: 1, Playlists: &playlists}
	err := db.Create(&playlistSnapshotConfiguration).Error

	if err != nil {
		return nil, err
	}

	// Because the ids changed from the ids defined of the request. We have to correct these ids in the database.
	for _, p := range playlistGroups {
		p.Playlist.PlaylistsOrder = getUpdatedPlaylistOrderIds(p, &playlistGroups)
	}

	err = db.Save(&playlists).Error

	if err != nil {
		return nil, err
	}

	r := CreateSnapshotResponse(playlistSnapshotConfiguration.ID, &playlists)
	return &r, nil
}

func CreateSnapshotResponse(snapshotID uint, dbPlaylists *[]*models.PlaylistSnapshot) model.PlaylistSnapshotConfiguration {
	r := model.PlaylistSnapshotConfiguration{
		ID:        fmt.Sprint(snapshotID),
		Playlists: createPlaylists(dbPlaylists),
	}

	return r

}

func createPlaylists(dbPlaylists *[]*models.PlaylistSnapshot) []*model.PlaylistSnapshot {
	playlists := []*model.PlaylistSnapshot{}

	for _, dbP := range *dbPlaylists {
		newP := model.PlaylistSnapshot{
			ID:                fmt.Sprint(dbP.ID),
			Name:              *dbP.Name,
			SpotifyPlaylistID: dbP.SpotifyPlaylistID,
			IsMixstack:        *dbP.IsMixStack,
			PlaylistOrder:     createPlaylistOrder(dbP.PlaylistsOrder),
			Associations:      createAssociations(dbP.Associations),
		}

		playlists = append(playlists, &newP)
	}

	return playlists
}

func createPlaylistOrder(playlistOrder *string) []*int {
	if *playlistOrder == "[]" {
		return []*int{}
	}

	playlistOrderIds := []*int{}

	b := []byte(*playlistOrder)
	err := json.Unmarshal(b, &playlistOrderIds)

	if err != nil {
		panic(err)
	}

	return playlistOrderIds
}

func createAssociations(dbAssociations *[]*models.PlaylistAssociationSnapshot) []*model.PlaylistAssociationSnapshot {
	associations := []*model.PlaylistAssociationSnapshot{}
	for _, a := range *dbAssociations {
		associations = append(associations, &model.PlaylistAssociationSnapshot{
			ID:               fmt.Sprint(a.ID),
			ChildPlaylistID:  fmt.Sprint(*a.ChildPlaylistID),
			ParentPlaylistID: fmt.Sprint(*a.ParentPlaylistID),
		})
	}

	return associations
}

func getUpdatedPlaylistOrderIds(group *playlistGroup, groups *[]*playlistGroup) *string {
	if len(group.PlaylistOrder) == 0 {
		return common.LiteralToPtr("[]")
	}

	playlistOrder := []uint{}
	for _, p := range group.PlaylistOrder {
		pId := uint(*p)
		for _, g := range *groups {
			if *g.InputID == pId {
				playlistOrder = append(playlistOrder, g.Playlist.ID)
			}
		}
	}

	bytes, err := json.Marshal(playlistOrder)

	if err != nil {
		panic(err)
	}

	s := string(bytes)
	return &s
}

func assignAssociations(p *model.NewPlaylistSnapshot, playlists *[]*models.PlaylistSnapshot) error {
	var err error
	associations := []*models.PlaylistAssociationSnapshot{}

	var playlistID uint
	if playlistID, err = common.StringToUint(p.PlaylistID, 32); err != nil {
		return err
	}

	playlist := getPlaylistById(&playlistID, playlists)

	for _, a := range p.Associations {
		var childPlaylistID uint
		var parentPlaylistID uint

		if childPlaylistID, err = common.StringToUint(a.ChildPlaylistID, 32); err != nil {
			return err
		}

		if parentPlaylistID, err = common.StringToUint(a.ParentPlaylistID, 32); err != nil {
			return err
		}

		// The children playlists and the parent will always create the same associations.
		// Because of that we can skip creating another association when iterating over the parent.
		// This will prevent duplicated associations in the database.
		if parentPlaylistID == playlistID {
			// Only playlists that are the parent are mix stacks
			getPlaylistById(&parentPlaylistID, playlists).IsMixStack = common.LiteralToPtr(true)
			continue
		}

		newA := models.PlaylistAssociationSnapshot{
			ChildPlaylist:  getPlaylistById(&childPlaylistID, playlists),
			ParentPlaylist: getPlaylistById(&parentPlaylistID, playlists),
		}
		associations = append(associations, &newA)
	}

	playlist.Associations = &associations
	return nil
}

func getPlaylistById(id *uint, playlists *[]*models.PlaylistSnapshot) *models.PlaylistSnapshot {
	for _, p := range *playlists {
		if *id == p.ID {
			return p
		}
	}

	panic(fmt.Sprintf("Could not find playlist with id: %v", *id))
}

func resetPlaylistIds(playlists *[]*models.PlaylistSnapshot) {
	for _, p := range *playlists {
		p.ID = 0
	}
}
