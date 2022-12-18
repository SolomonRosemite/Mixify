package mixify

import (
	"context"
	"fmt"

	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/SolomonRosemite/Mixify/internal/utils/common"
	"github.com/zmb3/spotify/v2"
)

func CreateOrUpdatePlaylists(client *spotify.Client, nodes *[]*PlaylistNode, playlists *[]*models.PlaylistSnapshot) (any, error) {
	user, err := client.CurrentUser(context.Background())
	if err != nil {
		return nil, err
	}

	spotifyPlaylists, err := getOrCreateSpotifyPlaylists(client, user, playlists)
	if err != nil {
		return nil, err
	}

	fmt.Println(len(*spotifyPlaylists))

	return "tbd", nil
}

func getOrCreateSpotifyPlaylists(client *spotify.Client, user *spotify.PrivateUser, playlists *[]*models.PlaylistSnapshot) (*[]*spotify.FullPlaylist, error) {
	spotifyPlaylists := make([]*spotify.FullPlaylist, len(*playlists))
	for i := 0; i < len(*playlists); i++ {
		var spotifyPlaylist *spotify.FullPlaylist
		var err error

		if (*playlists)[i].SpotifyPlaylistID == nil {
			spotifyPlaylist, err = createPlaylist(client, user, *(*playlists)[i].Name, "Playlist description")
		} else {
			playlistId := *(*playlists)[i].SpotifyPlaylistID
			spotifyPlaylistId := spotify.ID(playlistId)
			spotifyPlaylist, err = client.GetPlaylist(context.Background(), spotifyPlaylistId)
		}

		if err != nil {
			return nil, err
		}

		spotifyPlaylists[i] = spotifyPlaylist
		(*playlists)[i].SpotifyPlaylistID = common.LiteralToPtr(spotifyPlaylist.ID.String())
	}

	return &spotifyPlaylists, nil
}

func GetOrCreateSingleSpotifyPlaylists(client *spotify.Client, user *spotify.PrivateUser, playlistName string, playlistDescription string, spotifyPlaylistID *string) (*spotify.FullPlaylist, error) {
	var spotifyPlaylist *spotify.FullPlaylist
	var err error

	if spotifyPlaylistID == nil {
		spotifyPlaylist, err = createPlaylist(client, user, playlistName, playlistDescription)
	} else {
		spotifyPlaylistId := spotify.ID(*spotifyPlaylistID)
		spotifyPlaylist, err = client.GetPlaylist(context.Background(), spotifyPlaylistId)
	}

	if err != nil {
		return nil, err
	}

	return spotifyPlaylist, err
}

func createPlaylist(client *spotify.Client, user *spotify.PrivateUser, playlistName string, playlistDescription string) (*spotify.FullPlaylist, error) {
	createdPlaylist, err := client.CreatePlaylistForUser(context.Background(), user.ID, playlistName, playlistDescription, false, false)

	if err != nil {
		return nil, err
	}

	return createdPlaylist, nil
}
