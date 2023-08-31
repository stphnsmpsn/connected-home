use std::{net::SocketAddr, sync::Arc};

use common::error::ConnectedHomeResult;
use grpc::user::user_service_server::UserServiceServer;
use tonic::transport::Server;

use crate::{context::Context, user::handlers::MyUserService};

pub struct GrpcServer {}

impl GrpcServer {
    pub async fn serve(context: Arc<Context>) -> ConnectedHomeResult<()> {
        let bind_addr: SocketAddr = format!(
            "{}:{}",
            context.config.grpc_server.listen_address, context.config.grpc_server.port
        )
        .parse()?;

        let user_service = MyUserService::new(context.clone());

        let layer = tower::ServiceBuilder::new()
            .layer(grpc::RestoreTracingContextLayer {})
            .into_inner();

        Server::builder()
            .layer(layer)
            .add_service(UserServiceServer::new(user_service))
            .serve(bind_addr)
            .await?;
        Ok(())
    }
}
