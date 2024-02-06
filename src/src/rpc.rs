use service::{mixify_server::Mixify, OutputResponse, SnapshotRequest};

pub mod service {
    tonic::include_proto!("mixify");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("service_descriptor");
}

#[derive(Debug, Default)]
pub struct Service {}

#[tonic::async_trait]
impl Mixify for Service {
    async fn plan(
        &self,
        request: tonic::Request<SnapshotRequest>,
    ) -> std::result::Result<tonic::Response<OutputResponse>, tonic::Status> {
        return Ok(tonic::Response::new(OutputResponse {
            output: "".to_string(),
        }));
    }
    async fn apply(
        &self,
        request: tonic::Request<SnapshotRequest>,
    ) -> std::result::Result<tonic::Response<OutputResponse>, tonic::Status> {
        todo!()
    }
    async fn sync(
        &self,
        request: tonic::Request<SnapshotRequest>,
    ) -> std::result::Result<tonic::Response<OutputResponse>, tonic::Status> {
        todo!()
    }
}
