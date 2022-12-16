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
	UserID                          uint
	PlaylistConfigurationSnapshotID uint
}

type PlaylistConfigurationSnapshot struct {
	gorm.Model
	UserID    uint
	Playlists *[]*PlaylistSnapshot   `gorm:"foreignKey:PlaylistConfigurationSnapshotID"`
	Events    *[]*SyncPlaylistsEvent `gorm:"foreignKey:PlaylistConfigurationSnapshotID"`
}

type PlaylistSnapshot struct {
	gorm.Model
	Name                            *string
	PlaylistConfigurationSnapshotID *uint
	SpotifyPlaylistID               *string
	IsMixStack                      *bool
	PlaylistsOrder                  *string                         `gorm:"type:varchar(64);default:'[]';not null"`
	Associations                    *[]*PlaylistAssociationSnapshot `gorm:"ForeignKey:ChildPlaylistID;ParentPlaylistID"`
}

type PlaylistAssociationSnapshot struct {
	gorm.Model
	ChildPlaylistID  *uint
	ParentPlaylistID *uint
	ChildPlaylist    *PlaylistSnapshot
	ParentPlaylist   *PlaylistSnapshot
}

type PlaylistTrackAssociationSnapshot struct {
	gorm.Model
	PlaylistID *uint
	Playlist   *PlaylistSnapshot
	TrackID    *uint
	Track      *PlaylistTrackSnapshot
}

type PlaylistTrackSnapshot struct {
	gorm.Model
	SpotifyTrackID *string `gorm:"type:varchar(64)"`
}
