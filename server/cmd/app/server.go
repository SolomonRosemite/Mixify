package main

import (
	"os"

	"github.com/SolomonRosemite/Mixify/api/server"
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

	var db *gorm.DB

	if db, err = gorm.Open(postgres.Open(os.Getenv("DATABASE_DSN")), &gorm.Config{
		Logger: logger.Default.LogMode(logger.Info),
	}); err != nil {
		panic(err)
	}

	db.AutoMigrate(&models.PlaylistAssociationSnapshot{}, &models.PlaylistSnapshot{})

	if err = db.AutoMigrate(&models.PlaylistAssociationSnapshot{}, &models.PlaylistSnapshot{}); err != nil {
		panic(err)
	}

	server.StartServer()
}
