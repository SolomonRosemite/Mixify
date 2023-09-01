mod apply_command;
mod args;
mod constants;
mod new_command;
mod plan_command;
mod traits;

use clap::Parser;
use dotenv::dotenv;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

use crate::args::MixifyArgs;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    let mut builder = pretty_env_logger::env_logger::Builder::from_default_env();
    builder.target(pretty_env_logger::env_logger::Target::Stdout);
    builder.filter(Some("rspotify_http"), log::LevelFilter::Off);
    builder.init();

    // create_spotify_token().await;
    let spotify = create_client_from_token();

    let args = MixifyArgs::parse();
    let data = match args.entity_type {
        args::EntityType::New(cmd) => new_command::handle_new_snapshot(&cmd),
        args::EntityType::Plan(cmd) => plan_command::handle_plan_snapshot(&cmd),
        args::EntityType::Apply(cmd) => apply_command::handle_apply_snapshot(&cmd, &spotify).await,
    };

    match data {
        Ok(_) => {
            log::info!("Success!");
        }
        Err(e) => {
            log::error!("Error: {}", e);
        }
    }
}

async fn create_spotify_token() {
    let creds = Credentials::from_env().unwrap();

    let oauth = OAuth {
        redirect_uri: "http://localhost:8080/callback".to_string(),
        scopes: scopes!(
            "playlist-modify-public",
            "playlist-modify-private",
            "playlist-read-private",
            "playlist-read-collaborative",
            "user-read-currently-playing",
            "user-read-playback-state",
            "user-library-read",
            "user-read-private"
        ),
        ..Default::default()
    };
    let spotify = AuthCodeSpotify::new(creds, oauth);

    let url = spotify.get_authorize_url(true).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    let token = spotify.get_token();
    let idk = token.lock().await;
    let mut x = idk.unwrap();
    let y = x.take();
    let z = y.unwrap().access_token;
    println!("{:?}", z);

    std::env::set_var("TEST_TOKEN", z);
}

fn create_client_from_token() -> AuthCodeSpotify {
    let token_str = std::env::var("TEST_TOKEN").expect("TEST_TOKEN not set");
    let token = rspotify::model::Token {
        access_token: token_str.to_string(),
        refresh_token: None,
        expires_in: chrono::Duration::seconds(0),
        expires_at: Some(chrono::Utc::now()),
        scopes: scopes!(
            "playlist-read-private",
            "playlist-read-collaborative",
            "user-read-currently-playing",
            "user-read-playback-state",
            "user-library-read",
            "user-read-private"
        ),
    };

    return rspotify::AuthCodeSpotify::from_token(token);
}
