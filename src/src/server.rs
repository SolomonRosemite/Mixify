mod rpc;

use rpc::{
    service::{mixify_server::MixifyServer, FILE_DESCRIPTOR_SET},
    Service,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = Service::default();

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
