pub mod service {
    include!("proto/mixify.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("service_descriptor");
}

use std::result::Result;

use rspotify::clients::OAuthClient;
use service::mixify_server::Mixify;
use tonic::{Request, Response};

use crate::{traits::OptionExtension, types::Config};

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
        let state = self.auth_state().await;

        if state.is_err() || !state.as_ref().unwrap().0 {
            return Ok(Response::new(service::AuthStateResponse {
                status: service::LoginStatus::NotLoggedIn.into(),
                user_display_name: None,
            }));
        }

        Ok(Response::new(service::AuthStateResponse {
            status: service::LoginStatus::LoggedIn.into(),
            user_display_name: state.unwrap().1,
        }))
    }

    async fn create_token(
        &self,
        _: Request<service::Empty>,
    ) -> Result<Response<service::CreateTokenResponse>, tonic::Status> {
        return match self.spotify.get_authorize_url(false) {
            Ok(url) => {
                log::info!("Created token auth url: {}", url);
                Ok(Response::new(service::CreateTokenResponse {
                    url: url.to_string(),
                }))
            }
            Err(err) => {
                log::error!("Failed to create token: {}", err);
                Err(tonic::Status::internal("Failed to create token"))
            }
        };
    }

    type PlanStream = tonic::codec::Streaming<service::OutputResponse>;

    async fn plan(
        &self,
        request: Request<service::SnapshotRequest>,
    ) -> Result<Response<Self::PlanStream>, tonic::Status> {
        todo!()
    }

    async fn apply(
        &self,
        request: Request<service::SnapshotRequest>,
    ) -> Result<Response<service::OutputResponse>, tonic::Status> {
        todo!()
    }
    async fn sync(
        &self,
        request: Request<service::SnapshotRequest>,
    ) -> Result<Response<service::OutputResponse>, tonic::Status> {
        todo!()
    }
}

impl Service {
    async fn auth_state(&self) -> Result<(bool, Option<String>), anyhow::Error> {
        let token = self.spotify.token.clone();
        let mutex = token
            .lock()
            .await
            .map_err(|_| anyhow::anyhow!("Failed to get token"))?;
        let token = mutex.clone().or_error_str("Failed to get token")?;
        if token.is_expired() {
            return Ok((false, None));
        }

        let me = self.spotify.me().await?;
        Ok((true, me.display_name))
    }
}
