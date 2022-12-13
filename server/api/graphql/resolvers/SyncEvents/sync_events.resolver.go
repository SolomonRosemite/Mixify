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

	event, err := getPlaylistConfigById(db, &syncEventID, &userID)

	if err != nil {
		return nil, err
	}

	config := (*event.Configurations)[0]
	snapshotConfigRes := createPlaylistSnapshotConfiguration.CreateSnapshotResponse(config.ID, config.Playlists)
	res := model.SyncPlaylistsEvent{ID: fmt.Sprint(event.ID), UserID: fmt.Sprint(userID), ConfigurationSnapshot: &snapshotConfigRes}

	return &res, nil
}

func getPlaylistConfigById(db *gorm.DB, ID *uint, userID *uint) (*models.SyncPlaylistsEvent, error) {
	event := models.SyncPlaylistsEvent{}
	err := db.Preload("Configurations.Playlists.Associations").Model(&event).First(&event, "id = ? AND user_id = ?", ID, userID).Error
	return &event, err
}
