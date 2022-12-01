package models

type PlaylistSnapshot struct {
	Id                uint64
	TempTestName      string
	IsMixStack        bool
	SpotifyPlaylistId *string
	PlaylistsOrder    *string
	Associations      *[]*PlaylistAssociationSnapshot
}

type PlaylistAssociationSnapshot struct {
	Id               uint64
	ChildPlaylistId  *uint64
	ParentPlaylistId *uint64
}
