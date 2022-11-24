package mixify

import (
	"fmt"

	"github.com/SolomonRosemite/Mixify/internal/models"
)

// func CreateMixStackGraph(client *spotify.Client) {
func CreateMixStackGraph() {
	playlists, associations := createTestData()

	playlistIds := make([]uint64, len(*playlists))
	for i, playlist := range *playlists {
		playlistIds[i] = playlist.Id
	}

	start(&playlistIds, associations)
}

func start(playlistIds *[]uint64, associations *[]models.PlaylistAssociationSnapshot) {
	topLevelPlaylistIds := getAllTopLevelPlaylistIds(playlistIds, associations)
	list := createDependencyLists(&topLevelPlaylistIds, associations)
	createDependencyListToGraph(list)
}

func createDependencyListToGraph(list *[][][]uint64) {

}

func createDependencyLists(topLevelPlaylistIds *[]uint64, associations *[]models.PlaylistAssociationSnapshot) *[][][]uint64 {
	result := [][][]uint64{}

	fmt.Printf("number of top level playlists: %v\n", len(*topLevelPlaylistIds))
	for _, id := range *topLevelPlaylistIds {
		fmt.Printf("starting with top level playlist id: %v\n", id)
		res := createSingleDependencyList(id, associations, []uint64{})
		fmt.Printf("%v\n", res)
		result = append(result, res)
		fmt.Println("-----------------------------")
	}

	return &result
}

func createSingleDependencyList(playlistId uint64, associations *[]models.PlaylistAssociationSnapshot, visitedPlaylistIds []uint64) [][]uint64 {
	if nodeIsAlreadyVisited(playlistId, visitedPlaylistIds) {
		errorString := fmt.Sprintf("Circular playlist dependency detected. Tried to build playlist with id: %v.", playlistId)
		panic(errorString)
	}

	didFindChildren := false
	visitedPlaylistIds = append(visitedPlaylistIds, playlistId)
	returnVal := [][]uint64{}

	// If we have children, dfs these...
	for _, a := range *associations {
		if *a.ParentPlaylistId == playlistId {
			didFindChildren = true
			res := createSingleDependencyList(*a.ChildPlaylistId, associations, visitedPlaylistIds)
			returnVal = append(returnVal, res...)
		}
	}

	if !didFindChildren {
		returnVal = append(returnVal, visitedPlaylistIds)
	}

	return returnVal
}

func nodeIsAlreadyVisited(playlistId uint64, visitedPlaylistIds []uint64) bool {
	for _, a := range visitedPlaylistIds {
		if a == playlistId {
			return true
		}
	}
	return false
}

func getAllTopLevelPlaylistIds(playlistIds *[]uint64, associations *[]models.PlaylistAssociationSnapshot) []uint64 {
	topLevelPlaylistIds := []uint64{}
	for _, id := range *playlistIds {
		isParent := false
		isChild := false

		for _, a := range *associations {
			if *a.ChildPlaylistId == id {
				isChild = true
			}
			if *a.ParentPlaylistId == id {
				isParent = true
			}
		}

		if isParent && !isChild {
			topLevelPlaylistIds = append(topLevelPlaylistIds, id)
		}
	}

	return topLevelPlaylistIds
}

func createTestData() (*[]models.PlaylistSnapshot, *[]models.PlaylistAssociationSnapshot) {
	playlistAssociations := []models.PlaylistAssociationSnapshot{
		// Idk -> Second best lofi playlist eu
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
			Id:           4,
			TempTestName: "Generic Lofi",
			IsMixStack:   true,
			Associations: literalToPtr([]models.PlaylistAssociationSnapshot{playlistAssociations[1:2][0], playlistAssociations[2:3][0], playlistAssociations[4:5][0], playlistAssociations[5:6][0]}),
		},
		{
			Id:           5,
			TempTestName: "Best lofi playlist eu",
			IsMixStack:   true,
			Associations: literalToPtr([]models.PlaylistAssociationSnapshot{playlistAssociations[3:4][0], playlistAssociations[5:6][0]}),
		},
		{
			Id:           6,
			TempTestName: "Second best lofi playlist eu",
			IsMixStack:   true,
			Associations: literalToPtr([]models.PlaylistAssociationSnapshot{playlistAssociations[0:1][0], playlistAssociations[4:5][0], playlistAssociations[5:6][0]}),
		},
		{
			Id:           7,
			TempTestName: "Copy of \"Second best lofi playlist eu\" as test",
			IsMixStack:   true,
			Associations: literalToPtr([]models.PlaylistAssociationSnapshot{playlistAssociations[5:6][0]}),
		},
	}

	return &playlists, &playlistAssociations
}

func literalToPtr[T any](v T) *T {
	return &v
}
