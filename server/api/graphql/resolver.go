package graphql

import (
	"github.com/SolomonRosemite/Mixify/db"
	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/zmb3/spotify/v2"
)

// This file will not be regenerated automatically.
//
// It serves as dependency injection for your app, add any dependencies you require here.

type Resolver struct {
	// TODO: The expired codes should be removed from this map regularly
	EmailConfirmationCodes *map[string]*models.EmailConfirmation
	SpotifyUserAccess      *map[string]*spotify.Client
	DB                     *db.DBWrapper
}
