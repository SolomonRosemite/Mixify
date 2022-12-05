package models

import "gorm.io/gorm"

type PlaylistSnapshot struct {
	gorm.Model
	Name              *string
	IsMixStack        *bool
	SpotifyPlaylistID *string
	PlaylistsOrder    *string
	Associations      *[]*PlaylistAssociationSnapshot `gorm:"foreignKey:ChildPlaylistID;gorm:"foreignKey:ParentPlaylistID"`
}

type PlaylistAssociationSnapshot struct {
	gorm.Model
	ChildPlaylistID  *uint
	ParentPlaylistID *uint
	ChildPlaylist    *PlaylistSnapshot
	ParentPlaylist   *PlaylistSnapshot
}
