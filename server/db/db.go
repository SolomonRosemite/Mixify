package db

import (
	"gorm.io/gorm"
)

type DBWrapper struct {
	DB *gorm.DB
}
