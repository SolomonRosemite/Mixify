use rspotify::clients::OAuthClient;
use service::mixify_server::Mixify;

use crate::{traits::OptionExtension, types::Config};

pub mod service {
    include!("proto/mixify.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("service_descriptor");
}

#[derive(Debug)]
pub struct Service {
    pub spotify: rspotify::AuthCodeSpotify,
    pub config: Config,
}

#[tonic::async_trait]
impl Mixify for Service {
    async fn auth_state(
        &self,
        _: tonic::Request<service::Empty>,
    ) -> std::result::Result<tonic::Response<service::AuthStateResponse>, tonic::Status> {
        let state = self.auth_state().await;

        if state.is_err() || !state.as_ref().unwrap().0 {
            return Ok(tonic::Response::new(service::AuthStateResponse {
                status: service::LoginStatus::NotLoggedIn.into(),
                user_display_name: None,
            }));
        }

        Ok(tonic::Response::new(service::AuthStateResponse {
            status: service::LoginStatus::LoggedIn.into(),
            user_display_name: state.unwrap().1,
        }))
    }

    async fn create_token(
        &self,
        _: tonic::Request<service::Empty>,
    ) -> std::result::Result<tonic::Response<service::CreateTokenResponse>, tonic::Status> {
        return match self.spotify.get_authorize_url(false) {
            Ok(url) => {
                log::info!("Created token auth url: {}", url);
                Ok(tonic::Response::new(service::CreateTokenResponse {
                    url: url.to_string(),
                }))
            }
            Err(err) => {
                log::error!("Failed to create token: {}", err);
                Err(tonic::Status::internal("Failed to create token"))
            }
        };
    }

    async fn plan(
        &self,
        request: tonic::Request<service::SnapshotRequest>,
    ) -> std::result::Result<tonic::Response<Self::PlanStream>, tonic::Status> {
        todo!()
    }

    async fn apply(
        &self,
        request: tonic::Request<service::SnapshotRequest>,
    ) -> std::result::Result<tonic::Response<service::OutputResponse>, tonic::Status> {
        todo!()
    }
    async fn sync(
        &self,
        request: tonic::Request<service::SnapshotRequest>,
    ) -> std::result::Result<tonic::Response<service::OutputResponse>, tonic::Status> {
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

    // async fn create_token(&self) -> Result<String, tonic::Status> {}
}
