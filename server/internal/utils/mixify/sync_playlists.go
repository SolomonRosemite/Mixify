package mixify

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/SolomonRosemite/Mixify/internal/utils/common"
	"github.com/zmb3/spotify/v2"
	"gorm.io/gorm"
)

// TODO...
// 1. Refactor file (well the entire project is kinda a mess anyway)
// 2. Save changes to database
// 2.1 If a mix stack playlist never had a playlist id, assign one as a new playlist for that mix stack is created
// 2.2 Create track associations (This would be neat because it would allow to roll back a snapshot)
// 2.3 More?
// 3. Error handling (currently if the sync panics for some reason, the entire app crashes)
// 4. Even more??

type playlistTracksGroup struct {
	Tracks   *[]*spotify.PlaylistTrack
	playlist *models.PlaylistSnapshot
}

func StartSync(playlistConfigurationSnapshotID *uint, db *gorm.DB, client *spotify.Client) {
	snapshot, err := fetchPlaylists(playlistConfigurationSnapshotID, db, client)

	if err != nil {
		// TODO: Handle
		fmt.Println(err)
		return
	}

	nodes := CreateMixStackGraph(snapshot.Playlists)

	for _, n := range *nodes {
		_, err := n.buildPlaylistNode(snapshot.Playlists, client)

		if err != nil {
			// TODO: Handle...
			panic(err)
		}
	}
}

func fetchPlaylists(playlistConfigurationSnapshotID *uint, db *gorm.DB, client *spotify.Client) (*models.PlaylistConfigurationSnapshot, error) {
	res := models.PlaylistConfigurationSnapshot{}
	err := db.Model(&res).Preload("Playlists.Associations.ChildPlaylist").Preload("Playlists.Associations.ParentPlaylist").First(&res, "id = ?", playlistConfigurationSnapshotID).Error
	return &res, err
}

func (n *PlaylistNode) buildPlaylistNode(playlists *[]*models.PlaylistSnapshot, client *spotify.Client) (*playlistTracksGroup, error) {
	spotifyTracks := []*playlistTracksGroup{}
	if n.ChildrenNodes != nil {
		for _, childNode := range *n.ChildrenNodes {
			if childNode.PlaylistBuilt {
				continue
			}

			tracks, err := childNode.buildPlaylistNode(playlists, client)

			if err != nil {
				return nil, err
			}

			spotifyTracks = append(spotifyTracks, tracks)
		}
	}

	var playlist *models.PlaylistSnapshot
	for _, p := range *playlists {
		if p.ID == n.PlaylistId {
			playlist = p
		}
	}

	if playlist == nil {
		return nil, fmt.Errorf("playlist with id %v not found", n.PlaylistId)
	}

	return n.buildSinglePlaylist(playlist, playlists, &spotifyTracks, client)
}

func (n *PlaylistNode) buildSinglePlaylist(p *models.PlaylistSnapshot, playlists *[]*models.PlaylistSnapshot, tracks *[]*playlistTracksGroup, client *spotify.Client) (*playlistTracksGroup, error) {
	n.PlaylistBuilt = true
	if *p.IsMixStack {
		return n.buildMixStack(p, playlists, tracks, client)
	}
	return n.fetchTracksFromNormalPlaylist(p, client)
}

func (n *PlaylistNode) buildMixStack(p *models.PlaylistSnapshot, ps *[]*models.PlaylistSnapshot, groups *[]*playlistTracksGroup, client *spotify.Client) (*playlistTracksGroup, error) {
	var spotifyPlaylistID *spotify.ID
	var err error

	if p.SpotifyPlaylistID == nil {
		if spotifyPlaylistID, err = create(n, ps, client, p.SpotifyPlaylistID, *p.Name); err != nil {
			return nil, err
		}
	} else if _, err = client.GetPlaylist(context.Background(), spotify.ID(*p.SpotifyPlaylistID)); err != nil {
		if spotifyPlaylistID, err = create(n, ps, client, p.SpotifyPlaylistID, *p.Name); err != nil {
			return nil, err
		}
	} else {
		spotifyPlaylistID = common.LiteralToPtr(spotify.ID(*p.SpotifyPlaylistID))
	}

	tracksFromMixStackPlaylist, err := getSpotifyTracksFromPlaylistById(common.LiteralToPtr((*spotifyPlaylistID).String()), client)

	if err != nil {
		return nil, err
	}

	playlistOrder, err := getPlaylistOrder(*p.PlaylistsOrder)

	if err != nil {
		return nil, err
	}

	potentialTrackIDsToAdd := []spotify.ID{}
	for _, pID := range playlistOrder {
		for i := 0; i < len(*groups); i++ {
			if pID == (*groups)[i].playlist.ID {
				for j := 0; j < len(*(*groups)[i].Tracks); j++ {
					track := (*(*groups)[i].Tracks)[j]
					if !trackIDAlreadyExists(track.Track.ID, &potentialTrackIDsToAdd) {
						potentialTrackIDsToAdd = append(potentialTrackIDsToAdd, track.Track.ID)
					}
				}
				break
			}
		}
	}

	tracksToAdd := []spotify.ID{}
	for i := 0; i < len(potentialTrackIDsToAdd); i++ {
		if !trackAlreadyExists(potentialTrackIDsToAdd[i], tracksFromMixStackPlaylist) {
			tracksToAdd = append(tracksToAdd, potentialTrackIDsToAdd[i])
		}
	}

	startIndex := 0
	endIndex := 100
	for {
		if startIndex > len(tracksToAdd) {
			break
		}

		if endIndex > len(tracksToAdd) {
			endIndex = len(tracksToAdd)
		}

		batchOfTracksToAdd := tracksToAdd[startIndex:endIndex]
		if len(batchOfTracksToAdd) == 0 {
			break
		}

		_, err := client.AddTracksToPlaylist(context.Background(), *spotifyPlaylistID, batchOfTracksToAdd...)

		if err != nil {
			return nil, err
		}

		startIndex += 100
		endIndex += 100
	}

	// TODO: ?
	// fetch current playlist if existent
	// order songs by playlist order
	// add && remove songs
	// create spotify playlist if not existent
	// rename playlist (incase name changed)
	// create tracks && associations
	return nil, nil
}

func getPlaylistOrder(jsonContent string) ([]uint, error) {
	playlistOrder := []uint{}
	jsonData := []byte(jsonContent)
	err := json.Unmarshal(jsonData, &playlistOrder)

	return playlistOrder, err
}

func trackAlreadyExists(trackID spotify.ID, tracks *[]*spotify.PlaylistTrack) bool {
	for _, t := range *tracks {
		if t.Track.ID == trackID {
			return true
		}
	}

	return false
}

func trackIDAlreadyExists(trackID spotify.ID, tracks *[]spotify.ID) bool {
	for _, id := range *tracks {
		if id == trackID {
			return true
		}
	}

	return false
}

func (n *PlaylistNode) createDescription(playlists *[]*models.PlaylistSnapshot) string {

	var sb strings.Builder
	sb.WriteString("Generated mixstack using mixify. This playlist consists of:")

	numberOfChildren := len(*n.ChildrenNodes)
	for i := 0; i < numberOfChildren; i++ {
		node := (*n.ChildrenNodes)[i]
		playlist := getPlaylistById(node.PlaylistId, playlists)
		if numberOfChildren-1 == i {
			sb.WriteString(fmt.Sprintf(" %v.", *playlist.Name))
		} else {
			sb.WriteString(fmt.Sprintf(" %v x", *playlist.Name))
		}
	}

	return sb.String()
}

func getPlaylistById(ID uint, playlists *[]*models.PlaylistSnapshot) *models.PlaylistSnapshot {
	for _, p := range *playlists {
		if p.ID == ID {
			return p
		}
	}

	panic("playlist not found")
}

func create(n *PlaylistNode, ps *[]*models.PlaylistSnapshot, client *spotify.Client, spotifyPlaylistID *string, playlistName string) (*spotify.ID, error) {
	user, err := client.CurrentUser(context.Background())

	if err != nil {
		return nil, err
	}

	playlistDescription := n.createDescription(ps)
	playlist, err := GetOrCreateSingleSpotifyPlaylists(client, user, playlistName, playlistDescription, spotifyPlaylistID)

	if err != nil {
		return nil, err
	}

	return &playlist.ID, nil
}

func (n *PlaylistNode) fetchTracksFromNormalPlaylist(p *models.PlaylistSnapshot, client *spotify.Client) (*playlistTracksGroup, error) {
	// TODO...
	// fetch current playlist
	// create tracks && associations
	tracks, err := getSpotifyTracksFromPlaylistById(p.SpotifyPlaylistID, client)
	if err != nil {
		return nil, err
	}

	return &playlistTracksGroup{playlist: p, Tracks: tracks}, nil
}

func getSpotifyTracksFromPlaylistById(ID *string, client *spotify.Client) (*[]*spotify.PlaylistTrack, error) {
	spotifyPlaylistID := spotify.ID(*ID)
	tracks := []*spotify.PlaylistTrack{}
	p, err := client.GetPlaylist(context.Background(), spotifyPlaylistID)

	if err != nil {
		return nil, err
	}

	// If we could not get all tracks in one query, query all the missing once
	if p.Tracks.Total != len(p.Tracks.Tracks) {
		fetchedTracks, err := queryAllSongsFromPlaylist(&spotifyPlaylistID, client, p.Tracks.Tracks)

		if err != nil {
			return nil, err
		}

		tracks = *fetchedTracks
	} else {
		for i := 0; i < len(p.Tracks.Tracks); i++ {
			tracks = append(tracks, &p.Tracks.Tracks[i])
		}
	}

	return &tracks, nil
}

func queryAllSongsFromPlaylist(spotifyPlaylistID *spotify.ID, client *spotify.Client, initialTracks []spotify.PlaylistTrack) (*[]*spotify.PlaylistTrack, error) {
	tracks := []*spotify.PlaylistTrack{}

	for i := 0; i < len(initialTracks); i++ {
		tracks = append(tracks, &initialTracks[i])
	}

	offset := len(initialTracks)
	for {
		fetchedTracks, err := client.GetPlaylistTracks(context.Background(), *spotifyPlaylistID, spotify.Offset(offset))

		numberOfFetchedTracks := len(fetchedTracks.Tracks)
		if numberOfFetchedTracks == 0 {
			break
		}

		if err != nil {
			return nil, err
		}

		for i := 0; i < len(fetchedTracks.Tracks); i++ {
			tracks = append(tracks, &fetchedTracks.Tracks[i])
		}

		offset += numberOfFetchedTracks
	}

	return &tracks, nil
}
