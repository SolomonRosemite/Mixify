package graphql

import (
	"github.com/SolomonRosemite/Mixify/db"
	"github.com/zmb3/spotify/v2"
)

// This file will not be regenerated automatically.
//
// It serves as dependency injection for your app, add any dependencies you require here.

type Resolver struct {
	SpotifyUserAccess *map[string]*spotify.Client
	DB                *db.DBWrapper
}
