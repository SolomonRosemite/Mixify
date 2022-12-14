package syncEvents

import (
	"context"
	"fmt"

	createPlaylistSnapshotConfiguration "github.com/SolomonRosemite/Mixify/api/graphql/resolvers/CreatePlaylistSnapshotConfiguration"

	"github.com/SolomonRosemite/Mixify/api/graphql/model"
	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/SolomonRosemite/Mixify/internal/utils/common"
	"gorm.io/gorm"
)

// TODO: User authorization still missing.
func SyncEvents(ctx context.Context, id string, db *gorm.DB) (*model.SyncPlaylistsEvent, error) {
	syncEventID, err := common.StringToUint(id, 32)

	// TODO: Replace test user id
	userID := uint(1)

	if err != nil {
		return nil, err
	}

	event, configs, err := getPlaylistConfigById(db, &syncEventID, &userID)

	if err != nil {
		return nil, err
	}

	configSnapshots := []*model.PlaylistSnapshotConfiguration{}
	for _, c := range *configs {
		snapshotConfigRes := createPlaylistSnapshotConfiguration.CreateSnapshotResponse(c.ID, c.Playlists)
		configSnapshots = append(configSnapshots, &snapshotConfigRes)
	}

	res := model.SyncPlaylistsEvent{ID: fmt.Sprint(event.ID), UserID: fmt.Sprint(userID), ConfigurationSnapshot: configSnapshots}
	return &res, nil
}

func getPlaylistConfigById(db *gorm.DB, ID *uint, userID *uint) (*models.SyncPlaylistsEvent, *[]*models.PlaylistConfigurationSnapshot, error) {
	event := models.SyncPlaylistsEvent{}
	configurations := []*models.PlaylistConfigurationSnapshot{}

	if err := db.Model(&event).First(&event, "id = ? AND user_id = ?", ID, userID).Error; err != nil {
		return nil, nil, err
	}

	err := db.Preload("Playlists.Associations").Where("id = ?", event.PlaylistConfigurationSnapshotID).Find(&configurations).Error

	if err != nil {
		return nil, nil, err
	}

	return &event, &configurations, nil
}
