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
			Id:               0,
			ChildPlaylistId:  common.LiteralToPtr[uint64](0),
			ParentPlaylistId: common.LiteralToPtr[uint64](6),
		},
		// Lofi Vibes -> Generic Lofi
		{
			Id:               1,
			ChildPlaylistId:  common.LiteralToPtr[uint64](1),
			ParentPlaylistId: common.LiteralToPtr[uint64](4),
		},
		// Lofi Bangers -> Generic Lofi
		{
			Id:               2,
			ChildPlaylistId:  common.LiteralToPtr[uint64](2),
			ParentPlaylistId: common.LiteralToPtr[uint64](4),
		},
		// Top 30 Lofi songs -> Best lofi playlist eu
		{
			Id:               3,
			ChildPlaylistId:  common.LiteralToPtr[uint64](3),
			ParentPlaylistId: common.LiteralToPtr[uint64](5),
		},
		// Generic lofi -> Second best lofi playlist eu
		{
			Id:               4,
			ChildPlaylistId:  common.LiteralToPtr[uint64](4),
			ParentPlaylistId: common.LiteralToPtr[uint64](6),
		},
		// Generic lofi -> Best lofi playlist eu
		{
			Id:               5,
			ChildPlaylistId:  common.LiteralToPtr[uint64](4),
			ParentPlaylistId: common.LiteralToPtr[uint64](5),
		},
		// Second best lofi playlist eu -> Copy of Second best lofi playlist eu as test
		{
			Id:               6,
			ChildPlaylistId:  common.LiteralToPtr[uint64](6),
			ParentPlaylistId: common.LiteralToPtr[uint64](7),
		},
	}

	playlists := []*models.PlaylistSnapshot{
		{
			Id:                0,
			TempTestName:      "Random",
			IsMixStack:        false,
			SpotifyPlaylistId: common.LiteralToPtr("7I3t5Ebje4OsUDxuqCmsoG"),
			Associations:      common.LiteralToPtr(playlistAssociations[0:1]),
		},
		{
			Id:                1,
			TempTestName:      "Lofi Vibes",
			IsMixStack:        false,
			SpotifyPlaylistId: common.LiteralToPtr("4nLdH3m7SfCRfHOjfthUcF"),
			Associations:      common.LiteralToPtr(playlistAssociations[1:2]),
		},
		{
			Id:                2,
			TempTestName:      "Lofi Bangers",
			IsMixStack:        false,
			SpotifyPlaylistId: common.LiteralToPtr("4nLdH3m7SfCRfHOjfthUcF"),
			Associations:      common.LiteralToPtr(playlistAssociations[2:3]),
		},
		{
			Id:                3,
			TempTestName:      "Top 30 Lofi songs",
			IsMixStack:        false,
			SpotifyPlaylistId: common.LiteralToPtr("4nLdH3m7SfCRfHOjfthUcF"),
			Associations:      common.LiteralToPtr(playlistAssociations[3:4]),
		},
		{
			Id:                4,
			TempTestName:      "Generic Lofi",
			IsMixStack:        true,
			SpotifyPlaylistId: common.LiteralToPtr("3eO9NoZl7ZWL8w4k0TEPmH"),
			Associations:      common.LiteralToPtr([]*models.PlaylistAssociationSnapshot{playlistAssociations[1:2][0], playlistAssociations[2:3][0], playlistAssociations[4:5][0], playlistAssociations[5:6][0]}),
		},
		{
			Id:                5,
			TempTestName:      "Best lofi playlist eu",
			IsMixStack:        true,
			SpotifyPlaylistId: common.LiteralToPtr("1R0FUfQtxf77TMdDnvAt3F"),
			Associations:      common.LiteralToPtr([]*models.PlaylistAssociationSnapshot{playlistAssociations[3:4][0], playlistAssociations[5:6][0]}),
		},
		{
			Id:                6,
			TempTestName:      "Second best lofi playlist eu",
			IsMixStack:        true,
			SpotifyPlaylistId: common.LiteralToPtr("68JQu6CnTN0v4naDCqeMA4"),
			Associations:      common.LiteralToPtr([]*models.PlaylistAssociationSnapshot{playlistAssociations[0:1][0], playlistAssociations[4:5][0], playlistAssociations[5:6][0]}),
		},
		{
			Id:           7,
			TempTestName: "Copy of Second best lofi playlist eu as test",
			IsMixStack:   true,
			// SpotifyPlaylistId: shared.LiteralToPtr("4VdGOfXEmarZwj0q0oX25g"),
			Associations: common.LiteralToPtr([]*models.PlaylistAssociationSnapshot{playlistAssociations[6:7][0]}),
		},
	}

	return &playlists, &playlistAssociations
}
