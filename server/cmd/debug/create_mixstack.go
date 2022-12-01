package main

import (
	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/SolomonRosemite/Mixify/internal/utils/mixify"
	"github.com/SolomonRosemite/Mixify/internal/utils/spotify"
)

func main() {
	client, err := spotify.AuthenticateUser()
	if err != nil {
		panic(err)
	}

	playlists, associations := createTestData()
	nodes := mixify.CreateMixStackGraph(playlists, associations)
	_, err = mixify.CreateOrUpdatePlaylists(client, nodes, playlists)

	if err != nil {
		panic(err)
	}
}

func createTestData() (*[]models.PlaylistSnapshot, *[]models.PlaylistAssociationSnapshot) {
	playlistAssociations := []models.PlaylistAssociationSnapshot{
		// Random -> Second best lofi playlist eu
		{
			Id:               0,
			ChildPlaylistId:  literalToPtr[uint64](0),
			ParentPlaylistId: literalToPtr[uint64](6),
		},
		// Lofi Vibes -> Generic Lofi
		{
			Id:               1,
			ChildPlaylistId:  literalToPtr[uint64](1),
			ParentPlaylistId: literalToPtr[uint64](4),
		},
		// Lofi Bangers -> Generic Lofi
		{
			Id:               2,
			ChildPlaylistId:  literalToPtr[uint64](2),
			ParentPlaylistId: literalToPtr[uint64](4),
		},
		// Top 30 Lofi songs -> Best lofi playlist eu
		{
			Id:               3,
			ChildPlaylistId:  literalToPtr[uint64](3),
			ParentPlaylistId: literalToPtr[uint64](5),
		},
		// Generic lofi -> Second best lofi playlist eu
		{
			Id:               4,
			ChildPlaylistId:  literalToPtr[uint64](4),
			ParentPlaylistId: literalToPtr[uint64](6),
		},
		// Generic lofi -> Best lofi playlist eu
		{
			Id:               5,
			ChildPlaylistId:  literalToPtr[uint64](4),
			ParentPlaylistId: literalToPtr[uint64](5),
		},
		// Second best lofi playlist eu -> Copy of Second best lofi playlist eu as test
		{
			Id:               6,
			ChildPlaylistId:  literalToPtr[uint64](6),
			ParentPlaylistId: literalToPtr[uint64](7),
		},
	}

	playlists := []models.PlaylistSnapshot{
		{
			Id:                0,
			TempTestName:      "Random",
			IsMixStack:        false,
			SpotifyPlaylistId: literalToPtr("7I3t5Ebje4OsUDxuqCmsoG"),
			Associations:      literalToPtr(playlistAssociations[0:1]),
		},
		{
			Id:                1,
			TempTestName:      "Lofi Vibes",
			IsMixStack:        false,
			SpotifyPlaylistId: literalToPtr("4nLdH3m7SfCRfHOjfthUcF"),
			Associations:      literalToPtr(playlistAssociations[1:2]),
		},
		{
			Id:                2,
			TempTestName:      "Lofi Bangers",
			IsMixStack:        false,
			SpotifyPlaylistId: literalToPtr("4nLdH3m7SfCRfHOjfthUcF"),
			Associations:      literalToPtr(playlistAssociations[2:3]),
		},
		{
			Id:                3,
			TempTestName:      "Top 30 Lofi songs",
			IsMixStack:        false,
			SpotifyPlaylistId: literalToPtr("4nLdH3m7SfCRfHOjfthUcF"),
			Associations:      literalToPtr(playlistAssociations[3:4]),
		},
		{
			Id:                4,
			TempTestName:      "Generic Lofi",
			IsMixStack:        true,
			SpotifyPlaylistId: literalToPtr("3eO9NoZl7ZWL8w4k0TEPmH"),
			Associations:      literalToPtr([]models.PlaylistAssociationSnapshot{playlistAssociations[1:2][0], playlistAssociations[2:3][0], playlistAssociations[4:5][0], playlistAssociations[5:6][0]}),
		},
		{
			Id:                5,
			TempTestName:      "Best lofi playlist eu",
			IsMixStack:        true,
			SpotifyPlaylistId: literalToPtr("1R0FUfQtxf77TMdDnvAt3F"),
			Associations:      literalToPtr([]models.PlaylistAssociationSnapshot{playlistAssociations[3:4][0], playlistAssociations[5:6][0]}),
		},
		{
			Id:                6,
			TempTestName:      "Second best lofi playlist eu",
			IsMixStack:        true,
			SpotifyPlaylistId: literalToPtr("68JQu6CnTN0v4naDCqeMA4"),
			Associations:      literalToPtr([]models.PlaylistAssociationSnapshot{playlistAssociations[0:1][0], playlistAssociations[4:5][0], playlistAssociations[5:6][0]}),
		},
		{
			Id:                7,
			TempTestName:      "Copy of Second best lofi playlist eu as test",
			IsMixStack:        true,
			SpotifyPlaylistId: literalToPtr("4VdGOfXEmarZwj0q0oX25g"),
			Associations:      literalToPtr([]models.PlaylistAssociationSnapshot{playlistAssociations[6:7][0]}),
		},
	}

	return &playlists, &playlistAssociations
}

func literalToPtr[T any](v T) *T {
	return &v
}
