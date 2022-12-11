package db

import (
	"github.com/SolomonRosemite/Mixify/internal/models"
	"gorm.io/gorm"
)

type DBWrapper struct {
	DB *gorm.DB
}

func (dw *DBWrapper) GetLatestPlaylistConfig(userID uint) ([]*models.PlaylistSnapshot, error) {
	playlists := []*models.PlaylistSnapshot{}
	latestPlaylistConfig := []*models.PlaylistConfigurationSnapshot{}
	err := dw.DB.First(&latestPlaylistConfig, userID).Error

	if err != nil {
		return nil, err
	}

	err = dw.DB.Preload("Playlists.Associations").Order("created_at desc").First(&latestPlaylistConfig, "user_id = ?", userID).Error
	return playlists, err
}
