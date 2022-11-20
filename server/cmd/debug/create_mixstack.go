package main

import (
	"github.com/SolomonRosemite/Mixify/internal/untils/mixify"
	"github.com/SolomonRosemite/Mixify/internal/untils/spotify"
)

func main() {
	client, err := spotify.AuthenticateUser()
	if err != nil {
		panic(err)
	}
	mixify.CreateMixStackGraph(client)
}
