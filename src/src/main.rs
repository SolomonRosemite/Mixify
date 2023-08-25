mod apply_command;
mod args;
mod constants;
mod new_command;
mod plan_command;

use clap::Parser;
use rspotify::{prelude::*, scopes, Credentials};

use crate::args::MixifyArgs;

#[tokio::main]
async fn main() {
    let args = MixifyArgs::parse();

    match args.entity_type {
        args::EntityType::New(cmd) => new_command::handle_new_snapshot(&cmd),
        args::EntityType::Plan(cmd) => plan_command::handle_plan_snapshot(&cmd),
        args::EntityType::Apply(cmd) => apply_command::handle_apply_snapshot(&cmd),
    };

    return;

    // TODO: Use logger instead of println?
    env_logger::init();
    let creds = Credentials::from_env().unwrap();
    println!("{:?}", creds);

    // let oauth = OAuth {
    //     redirect_uri: "http://localhost:8080/callback".to_string(),
    //     scopes: scopes!("playlist-read-private", "playlist-read-collaborative", "user-read-currently-playing", "user-read-playback-state", "user-library-read", "user-read-private"),
    //     ..Default::default()
    // };
    // let spotify = AuthCodeSpotify::new(creds, oauth);
    let token_str = "";
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

    let spotify = rspotify::AuthCodeSpotify::from_token(token);

    // Obtaining the access token
    // let url = spotify.get_authorize_url(true).unwrap();
    // spotify.prompt_for_token(&url).await.unwrap();
    //
    // let token = spotify.get_token();
    // let idk = token.lock().await;
    // let mut x = idk.unwrap();
    // let y = x.take();
    // let z = y.unwrap().access_token;
    //
    // println!("{:?}", z);

    let user = spotify.me().await.expect("access token no longer valid???");
    println!("{:?}", user);
}
