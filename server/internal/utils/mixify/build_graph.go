package mixify

import (
	"fmt"

	"github.com/SolomonRosemite/Mixify/internal/models"
)

type playlistNode struct {
	PlaylistId       uint64
	TempPlaylistName string
	ChildrenNodes    *[]*playlistNode
}

// func CreateMixStackGraph(client *spotify.Client) {
func CreateMixStackGraph() {
	playlists, associations := createTestData()
	start(playlists, associations)
}

func start(playlists *[]models.PlaylistSnapshot, associations *[]models.PlaylistAssociationSnapshot) {
	playlistIds := make([]uint64, len(*playlists))
	for i, playlist := range *playlists {
		playlistIds[i] = playlist.Id
	}

	topLevelPlaylistIds := getAllTopLevelPlaylistIds(&playlistIds, associations)
	nodes := createDependencyGraph(&topLevelPlaylistIds, associations)

	for _, n := range *nodes {
		setPlaylistNames(playlists, n)
	}

	CreatePrettyGraph(playlistNode{PlaylistId: ^uint64(0), TempPlaylistName: "__root", ChildrenNodes: nodes})
}

func setPlaylistNames(playlists *[]models.PlaylistSnapshot, n *playlistNode) {
	for _, p := range *playlists {
		if p.Id == n.PlaylistId {
			n.TempPlaylistName = p.TempTestName
			break
		}
	}

	if n.ChildrenNodes == nil {
		return
	}

	for _, n := range *n.ChildrenNodes {
		setPlaylistNames(playlists, n)
	}
}

func createDependencyGraph(topLevelPlaylistIds *[]uint64, associations *[]models.PlaylistAssociationSnapshot) *[]*playlistNode {
	result := []*playlistNode{}

	fmt.Printf("number of top level playlists: %v\n", len(*topLevelPlaylistIds))
	for _, id := range *topLevelPlaylistIds {
		fmt.Printf("processing top level playlist with id: %v\n", id)
		res := createDependencyGraphForNode(playlistNode{PlaylistId: id}, associations, []uint64{})
		result = append(result, &res)
	}

	return &result
}

func createDependencyGraphForNode(node playlistNode, associations *[]models.PlaylistAssociationSnapshot, visitedPlaylistIds []uint64) playlistNode {
	if nodeIsAlreadyVisited(node, visitedPlaylistIds) {
		errorString := fmt.Sprintf("Circular playlist dependency detected. Tried to build playlist with id: %v.", node)
		panic(errorString)
	}

	visitedPlaylistIds = append(visitedPlaylistIds, node.PlaylistId)

	// If we have children, dfs these...
	for _, a := range *associations {
		if *a.ParentPlaylistId == node.PlaylistId {
			res := createDependencyGraphForNode(playlistNode{PlaylistId: *a.ChildPlaylistId}, associations, visitedPlaylistIds)
			visitedPlaylistIds = append(visitedPlaylistIds, *a.ChildPlaylistId)

			if node.ChildrenNodes == nil {
				node.ChildrenNodes = &[]*playlistNode{}
			}
			*node.ChildrenNodes = append(*node.ChildrenNodes, &res)
		}
	}

	return node
}

func nodeIsAlreadyVisited(node playlistNode, visitedPlaylistIds []uint64) bool {
	for _, a := range visitedPlaylistIds {
		if a == node.PlaylistId {
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
			TempTestName: "Copy of Second best lofi playlist eu as test",
			IsMixStack:   true,
			Associations: literalToPtr([]models.PlaylistAssociationSnapshot{playlistAssociations[5:6][0]}),
		},
	}

	return &playlists, &playlistAssociations
}

func literalToPtr[T any](v T) *T {
	return &v
}
