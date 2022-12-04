package mixify

import (
	"context"
	"fmt"
	"strings"

	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/SolomonRosemite/Mixify/internal/utils/common"
	"github.com/zmb3/spotify/v2"
)

func CreateOrUpdatePlaylists(client *spotify.Client, nodes *[]*playlistNode, playlists *[]*models.PlaylistSnapshot) (any, error) {
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
			spotifyPlaylist, err = createPlaylist(client, user, (*playlists)[i], playlists)
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

func createPlaylist(client *spotify.Client, user *spotify.PrivateUser, playlist *models.PlaylistSnapshot, playlists *[]*models.PlaylistSnapshot) (*spotify.FullPlaylist, error) {
	var sb strings.Builder
	sb.WriteString("Generated mixstack using mixify. This playlist consists of:")

	for _, a := range *playlist.Associations {
		var playlistName *string

		for _, p := range *playlists {
			if p.ID == *a.ChildPlaylistID {
				playlistName = &p.TempTestName
				break
			}
		}

		sb.WriteString(fmt.Sprintf(" %v", *playlistName))
	}
	sb.WriteString(".")

	playlistDescription := sb.String()
	createdPlaylist, err := client.CreatePlaylistForUser(context.Background(), user.ID, playlist.TempTestName, playlistDescription, false, false)

	if err != nil {
		return nil, err
	}

	return createdPlaylist, nil
}
