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
	spotifyUtil "github.com/SolomonRosemite/Mixify/internal/utils/spotify"
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

	srv := handler.NewDefaultServer(generated.NewExecutableSchema(generated.Config{Resolvers: &graphql.Resolver{
		DB: DB, SpotifyUserAccess: &spotifyUserAccessResolver}}))

	http.Handle("/playground", playground.Handler("GraphQL playground", "/query"))
	http.Handle("/query", srv)

	log.Printf("connect to http://localhost:%s/playground for GraphQL playground", port)

	// When running in docker this might have to be 0.0.0.0:port or just :port instead of localhost:port
	srvUrl := "localhost:" + port
	log.Fatal(http.ListenAndServe(srvUrl, nil))
}
