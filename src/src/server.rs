mod logger;
mod rpc;
mod traits;
mod types;

use dotenv::dotenv;
use logger::MemoryLogger;
use rpc::{
    service::{mixify_server::MixifyServer, FILE_DESCRIPTOR_SET},
    Service,
};
use tonic::transport::Server;
use traits::ResultExtension;
use types::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("Failed to load .env file");

    let memory_logger = MemoryLogger::new();
    let memory_logger = Box::new(memory_logger);

    let mut builder = pretty_env_logger::env_logger::Builder::from_default_env();
    builder.target(pretty_env_logger::env_logger::Target::Pipe(memory_logger));
    builder.filter(Some("rspotify_http"), log::LevelFilter::Off);
    builder.init();

    let config = parse_config().or_error_str("Following error occured when parsing config")?;
    let mut service = create_service(config);

    let addr = "[::1]:50051".parse()?;

    let server_reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(server_reflection)
        .add_service(MixifyServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

fn create_service(config: Config) -> Service {
    let creds = rspotify::Credentials::from_env().unwrap();
    let oauth = rspotify::OAuth {
        redirect_uri: "http://localhost:8080/callback".to_string(),
        scopes: rspotify::scopes!(
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

    let spotify = rspotify::AuthCodeSpotify::new(creds, oauth);
    return Service { spotify, config };
}

fn parse_config() -> Result<Config, anyhow::Error> {
    let allow_removing_songs = std::env::var("ALLOW_REMOVING_SONGS")
        .or_error_str("ALLOW_REMOVING_SONGS env var not set")?;

    let allow_removing_songs = match allow_removing_songs.as_str() {
        "true" => true,
        "false" => false,
        _ => {
            return Err(anyhow::anyhow!(format!(
                "Invalid value for ALLOW_REMOVING_SONGS. Expected 'true' or 'false' but got '{}'",
                allow_removing_songs
            )))
        }
    };

    let mixstack_suffix =
        std::env::var("MIXSTACK_SUFFIX").or_error_str("MIXSTACK_SUFFIX env var not set")?;
    let write_description = std::env::var("CREATE_PLAYLIST_DESCRIPTION")
        .or_error_str("CREATE_PLAYLIST_DESCRIPTION env var not set")?;

    let write_description = match write_description.as_str() {
        "true" => true,
        "false" => false,
        _ => {
            return Err(anyhow::anyhow!(format!(
                "Invalid value for CREATE_PLAYLIST_DESCRIPTION. Expected 'true' or 'false' but got '{}'",
                allow_removing_songs
            )))
        }
    };

    return Ok(Config {
        allow_removing_songs,
        mixstack_suffix,
        write_description,
    });
}
