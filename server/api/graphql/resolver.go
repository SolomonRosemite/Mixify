package graphql

import "github.com/SolomonRosemite/Mixify/db"

// This file will not be regenerated automatically.
//
// It serves as dependency injection for your app, add any dependencies you require here.

type Resolver struct {
	DB *db.DBWrapper
}
