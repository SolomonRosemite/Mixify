package graphql

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"

	"github.com/SolomonRosemite/Mixify/api/graphql/generated"
	"github.com/SolomonRosemite/Mixify/api/graphql/model"
	confirmConfirmationCode "github.com/SolomonRosemite/Mixify/api/graphql/resolvers/ConfirmConfirmationCode"
	createPlaylistSnapshotConfiguration "github.com/SolomonRosemite/Mixify/api/graphql/resolvers/CreatePlaylistSnapshotConfiguration"
	createSyncPlaylistsEvent "github.com/SolomonRosemite/Mixify/api/graphql/resolvers/CreateSyncPlaylistsEvent"
	requestConfirmationCode "github.com/SolomonRosemite/Mixify/api/graphql/resolvers/RequestConfirmationCode"
	syncEvents "github.com/SolomonRosemite/Mixify/api/graphql/resolvers/SyncEvents"
)

// CreateSyncPlaylistsEvent is the resolver for the createSyncPlaylistsEvent field.
func (r *mutationResolver) CreateSyncPlaylistsEvent(ctx context.Context, input model.NewSyncPlaylistsEvent) (*model.SyncPlaylistsEvent, error) {
	// TODO: Remove test user in the future
	return createSyncPlaylistsEvent.CreateSyncPlaylistsEvent(ctx, input, r.DB.DB, (*r.SpotifyUserAccess)["user_id:1"])
}

// CreatePlaylistSnapshotConfiguration is the resolver for the createPlaylistSnapshotConfiguration field.
func (r *mutationResolver) CreatePlaylistSnapshotConfiguration(ctx context.Context, input model.NewPlaylistSnapshotConfiguration) (*model.PlaylistSnapshotConfiguration, error) {
	return createPlaylistSnapshotConfiguration.CreatePlaylistSnapshotConfiguration(&ctx, &input, r.DB.DB)
}

// RequestConfirmationCode is the resolver for the requestConfirmationCode field.
func (r *queryResolver) RequestConfirmationCode(ctx context.Context, email string) (*model.RequestConfirmationCodeResponse, error) {
	return requestConfirmationCode.RequestConfirmationCode(ctx, email, r.EmailConfirmationCodes)
}

// ConfirmConfirmationCode is the resolver for the confirmConfirmationCode field.
func (r *queryResolver) ConfirmConfirmationCode(ctx context.Context, confirmationCode string, confirmationSecret *string) (*model.User, error) {
	return confirmConfirmationCode.ConfirmConfirmationCode(ctx, confirmationCode, *confirmationSecret, r.EmailConfirmationCodes, r.DB.DB)
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
