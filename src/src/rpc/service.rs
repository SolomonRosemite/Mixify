pub mod service {
    include!("../proto/mixify.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("service_descriptor");
}

use std::result::Result;

use rspotify::clients::OAuthClient;
use service::mixify_server::Mixify;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response};

use crate::rpc::echo;
use crate::traits::{OptionExtension, ResultExtension};
use crate::types::Config;

#[derive(Debug)]
pub struct Service {
    pub spotify: rspotify::AuthCodeSpotify,
    pub config: Config,
}

#[tonic::async_trait]
impl Mixify for Service {
    async fn auth_state(
        &self,
        _: Request<service::Empty>,
    ) -> Result<Response<service::AuthStateResponse>, tonic::Status> {
        if let Ok(me) = self.spotify.me().await {
            return Ok(Response::new(service::AuthStateResponse {
                status: service::LoginStatus::LoggedIn.into(),
                user: Some(service::User {
                    id: me.id.to_string(),
                    display_name: me.display_name,
                }),
            }));
        }

        return Ok(Response::new(service::AuthStateResponse {
            status: service::LoginStatus::NotLoggedIn.into(),
            user: None,
        }));
    }

    async fn create_token_url(
        &self,
        _: Request<service::Empty>,
    ) -> Result<Response<service::CreateTokenUrlResponse>, tonic::Status> {
        return match self.spotify.get_authorize_url(false) {
            Ok(url) => {
                log::info!("Created token auth url: {:?}", url);
                Ok(Response::new(service::CreateTokenUrlResponse { url }))
            }
            Err(err) => {
                log::error!("Failed to create token: {}", err);
                Err(tonic::Status::internal("Failed to create token"))
            }
        };
    }

    async fn submit_token_code(
        &self,
        request: tonic::Request<service::SubmitTokenRequestCode>,
    ) -> std::result::Result<tonic::Response<service::User>, tonic::Status> {
        let code = self
            .spotify
            .parse_response_code(&request.into_inner().url)
            .or_status_str("failed to parse code from redirect url")?;

        return match self.spotify.request_token(&code).await {
            Ok(_) => {
                let me =
                    self.spotify.me().await.or_status_str(
                        "congratulations! you made the impossible possible. you successfully authenticated, but failed to fetch user data. this should never happen",
                    )?;

                return Ok(Response::new(service::User {
                    id: me.id.to_string(),
                    display_name: me.display_name,
                }));
            }
            Err(err) => Err(tonic::Status::internal(err.to_string())),
        };
    }

    async fn plan(
        &self,
        request: Request<service::SnapshotRequest>,
    ) -> Result<Response<service::OutputResponse>, tonic::Status> {
        self.plan(request).await
    }

    type ApplyStream = ReceiverStream<Result<service::OutputResponse, tonic::Status>>;
    type SyncStream = ReceiverStream<Result<service::OutputResponse, tonic::Status>>;

    async fn apply(
        &self,
        _request: Request<service::SnapshotRequest>,
    ) -> Result<Response<Self::ApplyStream>, tonic::Status> {
        let (tx, rx) = mpsc::channel(4);
        echo::push_context(echo::GRPCOutputResponseSender::new(tx), || {
            echo::debug!("1");
            echo::info!("2");
            echo::warning!("3");
            echo::error!("4");
        });

        return Ok(Response::new(ReceiverStream::new(rx)));
    }

    async fn sync(
        &self,
        _request: Request<service::SnapshotRequest>,
    ) -> Result<Response<Self::SyncStream>, tonic::Status> {
        todo!()
    }
}
