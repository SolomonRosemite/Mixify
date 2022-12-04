package mixify

import (
	"fmt"

	"github.com/SolomonRosemite/Mixify/internal/models"
)

type playlistNode struct {
	PlaylistId       uint
	TempPlaylistName string
	ChildrenNodes    *[]*playlistNode
}

func CreateMixStackGraph(playlists *[]*models.PlaylistSnapshot, associations *[]*models.PlaylistAssociationSnapshot) *[]*playlistNode {
	return buildGraph(playlists, associations)
}

func buildGraph(playlists *[]*models.PlaylistSnapshot, associations *[]*models.PlaylistAssociationSnapshot) *[]*playlistNode {
	playlistIds := make([]uint, len(*playlists))
	for i, playlist := range *playlists {
		playlistIds[i] = playlist.Id
	}

	topLevelPlaylistIds := getAllTopLevelPlaylistIds(&playlistIds, associations)
	nodes := createDependencyGraph(&topLevelPlaylistIds, associations)

	for _, n := range *nodes {
		setPlaylistNames(playlists, n)
	}

	CreatePrettyGraph(playlistNode{PlaylistId: 0, TempPlaylistName: "__root", ChildrenNodes: nodes})

	return nodes
}

func setPlaylistNames(playlists *[]*models.PlaylistSnapshot, n *playlistNode) {
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

func createDependencyGraph(topLevelPlaylistIds *[]uint, associations *[]*models.PlaylistAssociationSnapshot) *[]*playlistNode {
	result := []*playlistNode{}

	fmt.Printf("number of top level playlists: %v\n", len(*topLevelPlaylistIds))
	for _, id := range *topLevelPlaylistIds {
		fmt.Printf("processing top level playlist with id: %v\n", id)
		res := createDependencyGraphForNode(playlistNode{PlaylistId: id}, associations, []uint{})
		result = append(result, &res)
	}

	return &result
}

func createDependencyGraphForNode(node playlistNode, associations *[]*models.PlaylistAssociationSnapshot, visitedPlaylistIds []uint) playlistNode {
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

func nodeIsAlreadyVisited(node playlistNode, visitedPlaylistIds []uint) bool {
	for _, a := range visitedPlaylistIds {
		if a == node.PlaylistId {
			return true
		}
	}
	return false
}

func getAllTopLevelPlaylistIds(playlistIds *[]uint, associations *[]*models.PlaylistAssociationSnapshot) []uint {
	topLevelPlaylistIds := []uint{}
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
