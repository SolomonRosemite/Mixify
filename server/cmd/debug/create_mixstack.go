package main

import (
	"github.com/SolomonRosemite/Mixify/internal/utils/mixify"
)

func main() {
	// client, err := spotify.AuthenticateUser()
	// if err != nil {
	// 	panic(err)
	// }
	// mixify.CreateMixStackGraph(client)
	mixify.CreateMixStackGraph()
}
