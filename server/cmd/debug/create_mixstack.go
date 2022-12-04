package main

import (
	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/SolomonRosemite/Mixify/internal/utils/common"
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

func createTestData() (*[]*models.PlaylistSnapshot, *[]*models.PlaylistAssociationSnapshot) {
	playlistAssociations := []*models.PlaylistAssociationSnapshot{
		// Random -> Second best lofi playlist eu
		{
			ID:               0,
			ChildPlaylistID:  common.LiteralToPtr[uint](0),
			ParentPlaylistID: common.LiteralToPtr[uint](6),
		},
		// Lofi Vibes -> Generic Lofi
		{
			ID:               1,
			ChildPlaylistID:  common.LiteralToPtr[uint](1),
			ParentPlaylistID: common.LiteralToPtr[uint](4),
		},
		// Lofi Bangers -> Generic Lofi
		{
			ID:               2,
			ChildPlaylistID:  common.LiteralToPtr[uint](2),
			ParentPlaylistID: common.LiteralToPtr[uint](4),
		},
		// Top 30 Lofi songs -> Best lofi playlist eu
		{
			ID:               3,
			ChildPlaylistID:  common.LiteralToPtr[uint](3),
			ParentPlaylistID: common.LiteralToPtr[uint](5),
		},
		// Generic lofi -> Second best lofi playlist eu
		{
			ID:               4,
			ChildPlaylistID:  common.LiteralToPtr[uint](4),
			ParentPlaylistID: common.LiteralToPtr[uint](6),
		},
		// Generic lofi -> Best lofi playlist eu
		{
			ID:               5,
			ChildPlaylistID:  common.LiteralToPtr[uint](4),
			ParentPlaylistID: common.LiteralToPtr[uint](5),
		},
		// Second best lofi playlist eu -> Copy of Second best lofi playlist eu as test
		{
			ID:               6,
			ChildPlaylistID:  common.LiteralToPtr[uint](6),
			ParentPlaylistID: common.LiteralToPtr[uint](7),
		},
	}

	playlists := []*models.PlaylistSnapshot{
		{
			ID:                0,
			TempTestName:      "Random",
			IsMixStack:        false,
			SpotifyPlaylistID: common.LiteralToPtr("7I3t5Ebje4OsUDxuqCmsoG"),
			Associations:      common.LiteralToPtr(playlistAssociations[0:1]),
		},
		{
			ID:                1,
			TempTestName:      "Lofi Vibes",
			IsMixStack:        false,
			SpotifyPlaylistID: common.LiteralToPtr("4nLdH3m7SfCRfHOjfthUcF"),
			Associations:      common.LiteralToPtr(playlistAssociations[1:2]),
		},
		{
			ID:                2,
			TempTestName:      "Lofi Bangers",
			IsMixStack:        false,
			SpotifyPlaylistID: common.LiteralToPtr("4nLdH3m7SfCRfHOjfthUcF"),
			Associations:      common.LiteralToPtr(playlistAssociations[2:3]),
		},
		{
			ID:                3,
			TempTestName:      "Top 30 Lofi songs",
			IsMixStack:        false,
			SpotifyPlaylistID: common.LiteralToPtr("4nLdH3m7SfCRfHOjfthUcF"),
			Associations:      common.LiteralToPtr(playlistAssociations[3:4]),
		},
		{
			ID:                4,
			TempTestName:      "Generic Lofi",
			IsMixStack:        true,
			SpotifyPlaylistID: common.LiteralToPtr("3eO9NoZl7ZWL8w4k0TEPmH"),
			Associations:      common.LiteralToPtr([]*models.PlaylistAssociationSnapshot{playlistAssociations[1:2][0], playlistAssociations[2:3][0], playlistAssociations[4:5][0], playlistAssociations[5:6][0]}),
		},
		{
			ID:                5,
			TempTestName:      "Best lofi playlist eu",
			IsMixStack:        true,
			SpotifyPlaylistID: common.LiteralToPtr("1R0FUfQtxf77TMdDnvAt3F"),
			Associations:      common.LiteralToPtr([]*models.PlaylistAssociationSnapshot{playlistAssociations[3:4][0], playlistAssociations[5:6][0]}),
		},
		{
			ID:                6,
			TempTestName:      "Second best lofi playlist eu",
			IsMixStack:        true,
			SpotifyPlaylistID: common.LiteralToPtr("68JQu6CnTN0v4naDCqeMA4"),
			Associations:      common.LiteralToPtr([]*models.PlaylistAssociationSnapshot{playlistAssociations[0:1][0], playlistAssociations[4:5][0], playlistAssociations[5:6][0]}),
		},
		{
			ID:           7,
			TempTestName: "Copy of Second best lofi playlist eu as test",
			IsMixStack:   true,
			// SpotifyPlaylistId: shared.LiteralToPtr("4VdGOfXEmarZwj0q0oX25g"),
			Associations: common.LiteralToPtr([]*models.PlaylistAssociationSnapshot{playlistAssociations[6:7][0]}),
		},
	}

	return &playlists, &playlistAssociations
}
