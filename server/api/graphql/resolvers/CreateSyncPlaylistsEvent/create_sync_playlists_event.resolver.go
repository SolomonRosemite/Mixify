package createSyncPlaylistsEvent

import (
	"context"
	"fmt"

	"github.com/SolomonRosemite/Mixify/api/graphql/model"
	createPlaylistSnapshotConfiguration "github.com/SolomonRosemite/Mixify/api/graphql/resolvers/CreatePlaylistSnapshotConfiguration"
	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/SolomonRosemite/Mixify/internal/utils/common"
	"gorm.io/gorm"
)

func CreateSyncPlaylistsEvent(ctx context.Context, input model.NewSyncPlaylistsEvent, db *gorm.DB) (*model.SyncPlaylistsEvent, error) {
	ID, err := common.StringToUint(input.SnapshotID, 32)
	// TODO: Replace test user id
	userID := uint(1)

	if err != nil {
		return nil, err
	}

	event, err := createEvent(&ID, &userID, db)
	if err != nil {
		return nil, err
	}

	config := (*event.Configurations)[0]
	snapshotConfigRes := createPlaylistSnapshotConfiguration.CreateSnapshotResponse(config.ID, config.Playlists)
	res := model.SyncPlaylistsEvent{ID: fmt.Sprint(event.ID), UserID: fmt.Sprint(userID), ConfigurationSnapshot: &snapshotConfigRes}

	return &res, nil
}

func createEvent(ID *uint, userID *uint, db *gorm.DB) (*models.SyncPlaylistsEvent, error) {
	config := models.PlaylistConfigurationSnapshot{}
	err := db.Preload("Playlists.Associations").Where("id = ? AND user_id = ?", ID, userID).First(&config).Error

	if err != nil {
		return nil, err
	}

	syncEvent := models.SyncPlaylistsEvent{UserID: *userID, Configurations: &[]*models.PlaylistConfigurationSnapshot{&config}}
	err = db.Save(&syncEvent).Error

	return &syncEvent, err
}
