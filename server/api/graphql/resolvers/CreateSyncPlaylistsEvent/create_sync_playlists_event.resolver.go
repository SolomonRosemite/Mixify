package createSyncPlaylistsEvent

import (
	"context"
	"fmt"

	"github.com/SolomonRosemite/Mixify/api/graphql/model"
	createPlaylistSnapshotConfiguration "github.com/SolomonRosemite/Mixify/api/graphql/resolvers/CreatePlaylistSnapshotConfiguration"
	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/SolomonRosemite/Mixify/internal/utils/common"
	"github.com/SolomonRosemite/Mixify/internal/utils/mixify"
	"github.com/zmb3/spotify/v2"
	"gorm.io/gorm"
)

func CreateSyncPlaylistsEvent(ctx context.Context, input model.NewSyncPlaylistsEvent, db *gorm.DB, client *spotify.Client) (*model.SyncPlaylistsEvent, error) {
	ID, err := common.StringToUint(input.ConfigurationSnapshotID, 32)
	// TODO: Replace test user id
	userID := uint(1)

	if err != nil {
		return nil, err
	}

	event, configs, err := createEvent(&ID, &userID, db, client)
	if err != nil {
		return nil, err
	}

	configSnapshots := []*model.PlaylistSnapshotConfiguration{}
	for _, c := range *configs {
		snapshotConfigRes := createPlaylistSnapshotConfiguration.CreateSnapshotResponse(c.ID, c.Playlists)
		configSnapshots = append(configSnapshots, &snapshotConfigRes)
	}

	res := model.SyncPlaylistsEvent{ID: fmt.Sprint(event.ID), UserID: fmt.Sprint(userID), ConfigurationSnapshot: configSnapshots}
	return &res, nil
}

func createEvent(ID *uint, userID *uint, db *gorm.DB, client *spotify.Client) (*models.SyncPlaylistsEvent, *[]*models.PlaylistConfigurationSnapshot, error) {
	configs := []*models.PlaylistConfigurationSnapshot{}
	err := db.Preload("Playlists.Associations").Where("id = ? AND user_id = ?", ID, userID).Find(&configs).Error

	if err != nil {
		return nil, nil, err
	}

	syncEvent := models.SyncPlaylistsEvent{UserID: *userID, PlaylistConfigurationSnapshotID: *ID}
	err = db.Save(&syncEvent).Error

	// Start sync event for creating and updating mix stacks
	go mixify.StartSync(ID, db, client)

	return &syncEvent, &configs, err
}
