package models

import "gorm.io/gorm"

type PlaylistSnapshot struct {
	gorm.Model
	TempTestName      string
	IsMixStack        bool
	SpotifyPlaylistID *string
	PlaylistsOrder    *string
	Associations      *[]*PlaylistAssociationSnapshot
}

type PlaylistAssociationSnapshot struct {
	gorm.Model
	ChildPlaylistID  *uint
	ParentPlaylistID *uint
}
