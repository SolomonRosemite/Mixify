package models

import "gorm.io/gorm"

type User struct {
	gorm.Model
	SpotifyUserID *string
	Email         *string
	Username      *string
	SyncEvents    *[]*SyncPlaylistsEvent
}

type SyncPlaylistsEvent struct {
	gorm.Model
	UserID         uint
	Configurations *[]*PlaylistConfigurationSnapshot `gorm:"foreignKey:SyncPlaylistsEventID"`
}

type PlaylistConfigurationSnapshot struct {
	gorm.Model
	SyncPlaylistsEventID uint
	UserID               uint
	Playlists            *[]*PlaylistSnapshot `gorm:"foreignKey:SnapshotID"`
}

type PlaylistSnapshot struct {
	gorm.Model
	Name              *string
	SnapshotID        *uint
	SpotifyPlaylistID *string
	IsMixStack        *bool
	PlaylistsOrder    *string
	Associations      *[]*PlaylistAssociationSnapshot `gorm:"ForeignKey:ChildPlaylistID;ParentPlaylistID"`
}

type PlaylistAssociationSnapshot struct {
	gorm.Model
	ChildPlaylistID  *uint
	ParentPlaylistID *uint
	ChildPlaylist    *PlaylistSnapshot
	ParentPlaylist   *PlaylistSnapshot
}
