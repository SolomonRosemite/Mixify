package main

import (
	"os"

	"github.com/SolomonRosemite/Mixify/api/server"
	"github.com/SolomonRosemite/Mixify/db"
	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/joho/godotenv"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

func main() {
	err := godotenv.Load("dev.env")

	if err != nil {
		panic(err)
	}

	var gormDB *gorm.DB

	if gormDB, err = gorm.Open(postgres.Open(os.Getenv("DATABASE_DSN")), &gorm.Config{
		Logger: logger.Default.LogMode(logger.Info),
	}); err != nil {
		panic(err)
	}

	if err = gormDB.AutoMigrate(&models.User{}, &models.PlaylistConfigurationSnapshot{}, &models.SyncPlaylistsEvent{}, &models.PlaylistAssociationSnapshot{}, &models.PlaylistSnapshot{}, &models.PlaylistTrackSnapshot{}, &models.PlaylistTrackAssociationSnapshot{}); err != nil {
		panic(err)
	}

	server.StartServer(&db.DBWrapper{DB: gormDB})
}
