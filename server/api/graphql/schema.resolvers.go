package graphql

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"
	"fmt"

	"github.com/SolomonRosemite/Mixify/api/graphql/generated"
	"github.com/SolomonRosemite/Mixify/api/graphql/model"
	createPlaylistSnapshotConfiguration "github.com/SolomonRosemite/Mixify/api/graphql/resolvers/CreatePlaylistSnapshotConfiguration"
)

// CreatePlaylistSnapshotConfiguration is the resolver for the createPlaylistSnapshotConfiguration field.
func (r *mutationResolver) CreatePlaylistSnapshotConfiguration(ctx context.Context, input model.NewPlaylistSnapshotConfiguration) (*model.PlaylistSnapshotConfiguration, error) {
	return createPlaylistSnapshotConfiguration.CreatePlaylistSnapshotConfiguration(&ctx, &input, r.DB.DB)
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
