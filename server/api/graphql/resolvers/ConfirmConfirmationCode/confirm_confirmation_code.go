package confirmConfirmationCode

import (
	"context"
	"fmt"
	"time"

	"github.com/SolomonRosemite/Mixify/api/graphql/model"
	"github.com/SolomonRosemite/Mixify/internal/models"
	"gorm.io/gorm"
)

func ConfirmConfirmationCode(ctx context.Context, confirmationCode string, confirmationSecret string, codes *map[string]*models.EmailConfirmation, db *gorm.DB) (*model.User, error) {
	if result, found := (*codes)[confirmationCode+confirmationSecret]; found {
		if time.Now().After(*result.Expiration) {
			return nil, fmt.Errorf("confirmation code expired. please request to send a new code")
		}

		delete(*codes, confirmationCode+confirmationSecret)
		user, err := createOrGetUser(*result.Email, db)
		return user, err
	} else {
		return nil, fmt.Errorf("invalid confirmation code")
	}
}

func createOrGetUser(email string, db *gorm.DB) (*model.User, error) {
	var user models.User
	err := db.Where("email = ?", email).First(&user).Error

	if err == nil {
		return &model.User{
			ID:    fmt.Sprint(user.ID),
			Email: *user.Email,
		}, nil
	}

	err = db.Create(&models.User{
		Email: &email,
	}).Error

	if err != nil {
		return nil, err
	}

	return &model.User{
		ID:    fmt.Sprint(user.ID),
		Email: *user.Email,
	}, nil
}
