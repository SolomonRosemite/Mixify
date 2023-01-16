package server

import (
	"log"
	"net/http"
	"os"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/SolomonRosemite/Mixify/api/graphql"
	"github.com/SolomonRosemite/Mixify/api/graphql/generated"
	"github.com/SolomonRosemite/Mixify/db"
	"github.com/SolomonRosemite/Mixify/internal/models"
	spotifyUtil "github.com/SolomonRosemite/Mixify/internal/utils/spotify"
	"github.com/go-chi/chi"
	"github.com/rs/cors"
	"github.com/zmb3/spotify/v2"
)

const defaultPort = "5000"

func StartServer(DB *db.DBWrapper) {
	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}

	spotifyUserAccessResolver := make(map[string]*spotify.Client)

	// TODO: Remove test user in the future
	user, err := spotifyUtil.AuthenticateUser()

	if err != nil {
		panic(user)
	}

	spotifyUserAccessResolver["user_id:1"] = user

	router := chi.NewRouter()

	router.Use(cors.New(cors.Options{
		AllowedOrigins:   []string{"http://localhost:3000", "http://127.0.0.1:3000"},
		AllowCredentials: true,
		Debug:            true,
	}).Handler)

	srv := handler.NewDefaultServer(generated.NewExecutableSchema(generated.Config{Resolvers: &graphql.Resolver{
		DB: DB, SpotifyUserAccess: &spotifyUserAccessResolver, EmailConfirmationCodes: &map[string]*models.EmailConfirmation{}}}))

	router.Handle("/playground", playground.Handler("GraphQL playground", "/query"))
	router.Handle("/query", srv)

	log.Printf("connect to http://localhost:%s/playground for GraphQL playground", port)

	// When running in docker this might have to be 0.0.0.0:port or just :port instead of localhost:port
	srvUrl := "localhost:" + port
	log.Fatal(http.ListenAndServe(srvUrl, router))
}
