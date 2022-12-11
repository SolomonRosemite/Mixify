package mixify

import (
	"fmt"

	"github.com/SolomonRosemite/Mixify/internal/models"
)

type PlaylistNode struct {
	PlaylistId    uint
	Name          string
	ChildrenNodes *[]*PlaylistNode
}

func CreateMixStackGraph(playlists *[]*models.PlaylistSnapshot, associations *[]*models.PlaylistAssociationSnapshot) *[]*PlaylistNode {
	return buildGraph(playlists, associations)
}

func buildGraph(playlists *[]*models.PlaylistSnapshot, associations *[]*models.PlaylistAssociationSnapshot) *[]*PlaylistNode {
	playlistIds := make([]uint, len(*playlists))
	for i, playlist := range *playlists {
		playlistIds[i] = playlist.ID
	}

	topLevelPlaylistIds := getAllTopLevelPlaylistIds(&playlistIds, associations)
	nodes := createDependencyGraph(&topLevelPlaylistIds, associations)

	for _, n := range *nodes {
		setPlaylistNames(playlists, n)
	}

	CreatePrettyGraph(PlaylistNode{PlaylistId: 0, Name: "__root", ChildrenNodes: nodes})

	return nodes
}

func setPlaylistNames(playlists *[]*models.PlaylistSnapshot, n *PlaylistNode) {
	for _, p := range *playlists {
		if p.ID == n.PlaylistId {
			n.Name = *p.Name
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

func createDependencyGraph(topLevelPlaylistIds *[]uint, associations *[]*models.PlaylistAssociationSnapshot) *[]*PlaylistNode {
	result := []*PlaylistNode{}

	fmt.Printf("number of top level playlists: %v\n", len(*topLevelPlaylistIds))
	for _, id := range *topLevelPlaylistIds {
		fmt.Printf("processing top level playlist with id: %v\n", id)
		res := createDependencyGraphForNode(PlaylistNode{PlaylistId: id}, associations, []uint{})
		result = append(result, &res)
	}

	return &result
}

func createDependencyGraphForNode(node PlaylistNode, associations *[]*models.PlaylistAssociationSnapshot, visitedPlaylistIds []uint) PlaylistNode {
	if nodeIsAlreadyVisited(node, visitedPlaylistIds) {
		errorString := fmt.Sprintf("Circular playlist dependency detected. Tried to build playlist with id: %v.", node)
		panic(errorString)
	}

	visitedPlaylistIds = append(visitedPlaylistIds, node.PlaylistId)

	// If we have children, dfs these...
	for _, a := range *associations {
		if *a.ParentPlaylistID == node.PlaylistId {
			res := createDependencyGraphForNode(PlaylistNode{PlaylistId: *a.ChildPlaylistID}, associations, visitedPlaylistIds)
			visitedPlaylistIds = append(visitedPlaylistIds, *a.ChildPlaylistID)

			if node.ChildrenNodes == nil {
				node.ChildrenNodes = &[]*PlaylistNode{}
			}
			*node.ChildrenNodes = append(*node.ChildrenNodes, &res)
		}
	}

	return node
}

func nodeIsAlreadyVisited(node PlaylistNode, visitedPlaylistIds []uint) bool {
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
			if *a.ChildPlaylistID == id {
				isChild = true
			}
			if *a.ParentPlaylistID == id {
				isParent = true
			}
		}

		if isParent && !isChild {
			topLevelPlaylistIds = append(topLevelPlaylistIds, id)
		}
	}

	return topLevelPlaylistIds
}
