package mixify

import (
	"os"

	"github.com/dominikbraun/graph"
	"github.com/dominikbraun/graph/draw"
)

func CreatePrettyGraph(startingNode PlaylistNode) {

	g := graph.New(graph.StringHash, graph.Directed())

	createVertices(&startingNode, g)
	createEdges(&startingNode, g)

	file, _ := os.Create("graph.gv")
	_ = draw.DOT(g, file)
}

func createVertices(n *PlaylistNode, g graph.Graph[string, string]) {
	err := g.AddVertex(n.Name)

	if err != nil {
		panic(err)
	}

	if n.ChildrenNodes == nil {
		return
	}

	for _, cn := range *n.ChildrenNodes {
		createVertices(cn, g)
	}
}

func createEdges(n *PlaylistNode, g graph.Graph[string, string]) {
	if n.ChildrenNodes == nil {
		return
	}

	for _, cn := range *n.ChildrenNodes {
		_ = g.AddEdge(cn.Name, n.Name)
	}

	for _, cn := range *n.ChildrenNodes {
		if cn.ChildrenNodes == nil {
			continue
		}

		createEdges(cn, g)
	}
}
