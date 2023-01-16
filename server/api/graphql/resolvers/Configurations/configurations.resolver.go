package configurations

import (
	"context"

	"github.com/SolomonRosemite/Mixify/api/graphql/model"
	createPlaylistSnapshotConfiguration "github.com/SolomonRosemite/Mixify/api/graphql/resolvers/CreatePlaylistSnapshotConfiguration"
	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/SolomonRosemite/Mixify/internal/utils/common"
	"gorm.io/gorm"
)

func Configurations(ctx context.Context, id string, db *gorm.DB) (*model.PlaylistSnapshotConfiguration, error) {
	configID, err := common.StringToUint(id, 32)

	// TODO: Replace test user id
	userID := uint(1)

	if err != nil {
		return nil, err
	}

	config, err := getPlaylistConfigById(db, &configID, &userID)

	if err != nil {
		return nil, err
	}

	c := createPlaylistSnapshotConfiguration.CreateSnapshotResponse(config.ID, config.Playlists)
	return &c, nil
}

func getPlaylistConfigById(db *gorm.DB, ID *uint, userID *uint) (*models.PlaylistConfigurationSnapshot, error) {
	configuration := &models.PlaylistConfigurationSnapshot{}

	err := db.Preload("Playlists.Associations").Where("id = ?", ID).Find(&configuration).Error

	if err != nil {
		return nil, err
	}

	return configuration, nil
}
