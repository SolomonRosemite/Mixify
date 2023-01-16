package requestAccessToken

import (
	"context"

	"github.com/SolomonRosemite/Mixify/api/graphql/model"
	"github.com/zmb3/spotify/v2"
)

func RequestAccessToken(ctx context.Context, client *spotify.Client) (*model.RequestAccessTokenResponse, error) {
	token, err := client.Token()

	if err != nil {
		return nil, err
	}

	return &model.RequestAccessTokenResponse{AccessToken: token.AccessToken, ExpiresIn: token.Expiry.String()}, nil
}
