package graphql

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"

	"github.com/SolomonRosemite/Mixify/api/graphql/generated"
	"github.com/SolomonRosemite/Mixify/api/graphql/model"
	createPlaylistSnapshotConfiguration "github.com/SolomonRosemite/Mixify/api/graphql/resolvers/CreatePlaylistSnapshotConfiguration"
	createSyncPlaylistsEvent "github.com/SolomonRosemite/Mixify/api/graphql/resolvers/CreateSyncPlaylistsEvent"
	syncEvents "github.com/SolomonRosemite/Mixify/api/graphql/resolvers/SyncEvents"
)

// CreateSyncPlaylistsEvent is the resolver for the createSyncPlaylistsEvent field.
func (r *mutationResolver) CreateSyncPlaylistsEvent(ctx context.Context, input model.NewSyncPlaylistsEvent) (*model.SyncPlaylistsEvent, error) {
	return createSyncPlaylistsEvent.CreateSyncPlaylistsEvent(ctx, input, r.DB.DB)
}

// CreatePlaylistSnapshotConfiguration is the resolver for the createPlaylistSnapshotConfiguration field.
func (r *mutationResolver) CreatePlaylistSnapshotConfiguration(ctx context.Context, input model.NewPlaylistSnapshotConfiguration) (*model.PlaylistSnapshotConfiguration, error) {
	return createPlaylistSnapshotConfiguration.CreatePlaylistSnapshotConfiguration(&ctx, &input, r.DB.DB)
}

// SyncEvents is the resolver for the syncEvents field.
func (r *queryResolver) SyncEvents(ctx context.Context, id string) (*model.SyncPlaylistsEvent, error) {
	return syncEvents.SyncEvents(ctx, id, r.DB.DB)
}

// Mutation returns generated.MutationResolver implementation.
func (r *Resolver) Mutation() generated.MutationResolver { return &mutationResolver{r} }

// Query returns generated.QueryResolver implementation.
func (r *Resolver) Query() generated.QueryResolver { return &queryResolver{r} }

type mutationResolver struct{ *Resolver }
type queryResolver struct{ *Resolver }
