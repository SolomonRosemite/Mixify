package models

import "time"

type EmailConfirmation struct {
	Email      *string
	Expiration *time.Time
	Code       *string
}
