package models

type PlaylistSnapshot struct {
	Id                uint
	TempTestName      string
	IsMixStack        bool
	SpotifyPlaylistId *string
	PlaylistsOrder    *string
	Associations      *[]*PlaylistAssociationSnapshot
}

type PlaylistAssociationSnapshot struct {
	Id               uint
	ChildPlaylistId  *uint
	ParentPlaylistId *uint
}
