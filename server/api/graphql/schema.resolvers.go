package graphql

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"
	"fmt"

	"github.com/SolomonRosemite/Mixify/api/graphql/generated"
	"github.com/SolomonRosemite/Mixify/api/graphql/model"
	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/SolomonRosemite/Mixify/internal/utils/common"
	"github.com/SolomonRosemite/Mixify/internal/utils/mixify"
	"gorm.io/gorm"
)

// CreatePlaylistSnapshotConfiguration is the resolver for the createPlaylistSnapshotConfiguration field.
func (r *mutationResolver) CreatePlaylistSnapshotConfiguration(ctx context.Context, input model.NewPlaylistSnapshotConfiguration) (*model.PlaylistSnapshotConfiguration, error) {
	playlists := []*models.PlaylistSnapshot{}
	association := []*models.PlaylistAssociationSnapshot{}

	for _, p := range input.Playlists {
		var err error
		var playlistID uint
		if playlistID, err = common.StringToUint(p.PlaylistID, 32); err != nil {
			return nil, err
		}

		newP := models.PlaylistSnapshot{
			Model:             gorm.Model{ID: playlistID},
			Name:              &p.Name,
			SpotifyPlaylistID: p.SpotifyPlaylistID,
			PlaylistsOrder:    p.PlaylistOrder,
		}

		playlists = append(playlists, &newP)

		for _, a := range p.Associations {
			var childPlaylistID uint
			var parentPlaylistID uint

			if childPlaylistID, err = common.StringToUint(a.ChildPlaylistID, 32); err != nil {
				return nil, err
			}

			if parentPlaylistID, err = common.StringToUint(a.ParentPlaylistID, 32); err != nil {
				return nil, err
			}

			exists := combinationAlreadyExist(&childPlaylistID, &parentPlaylistID, &association)
			if exists {
				continue
			}

			newA := models.PlaylistAssociationSnapshot{ChildPlaylistID: &childPlaylistID, ParentPlaylistID: &parentPlaylistID}
			association = append(association, &newA)

		}
	}

	res := mixify.CreateMixStackGraph(&playlists, &association)
	fmt.Printf("%v", res)

	// TODO: Add to database
	// TODO: Update schema and return

	return &model.PlaylistSnapshotConfiguration{}, nil
}

func combinationAlreadyExist(childPlaylistID *uint, parentPlaylistID *uint, association *[]*models.PlaylistAssociationSnapshot) bool {
	for _, a := range *association {
		if *a.ChildPlaylistID == *childPlaylistID && *a.ParentPlaylistID == *parentPlaylistID {
			return true
		}
	}

	return false
}

// SyncLogs is the resolver for the syncLogs field.
func (r *queryResolver) SyncLogs(ctx context.Context) ([]*model.SyncLog, error) {
	testUserId := uint(1)
	data, err := r.DB.GetLatestPlaylistConfig(testUserId)

	if err != nil {
		return nil, err
	}

	fmt.Printf("%v\n", data)
	return []*model.SyncLog{}, nil
}

// Mutation returns generated.MutationResolver implementation.
func (r *Resolver) Mutation() generated.MutationResolver { return &mutationResolver{r} }

// Query returns generated.QueryResolver implementation.
func (r *Resolver) Query() generated.QueryResolver { return &queryResolver{r} }

type mutationResolver struct{ *Resolver }
type queryResolver struct{ *Resolver }
