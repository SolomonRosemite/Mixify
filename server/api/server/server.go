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
)

const defaultPort = "8080"

func StartServer(DB *db.DBWrapper) {
	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}

	srv := handler.NewDefaultServer(generated.NewExecutableSchema(generated.Config{Resolvers: &graphql.Resolver{DB: DB}}))

	http.Handle("/playground", playground.Handler("GraphQL playground", "/query"))
	http.Handle("/query", srv)

	log.Printf("connect to http://localhost:%s/playground for GraphQL playground", port)

	// When running in docker this might have to be 0.0.0.0:port or just :port instead of localhost:port
	srvUrl := "localhost:" + port
	log.Fatal(http.ListenAndServe(srvUrl, nil))
}
