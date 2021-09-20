use crate::usecase::port::{CreateUserInputData, CreateUserPort};
use anyhow::Result;
use kzmake_rca_user_v1::user_service_server::{UserService, UserServiceServer};
use kzmake_rca_user_v1::User as ProtoUser;
use kzmake_rca_user_v1::{CreateRequest, CreateResponse};
use kzmake_rca_user_v1::{DeleteRequest, DeleteResponse};
use kzmake_rca_user_v1::{GetRequest, GetResponse};
use kzmake_rca_user_v1::{ListRequest, ListResponse};
use kzmake_rca_user_v1::{RenameRequest, RenameResponse};
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

pub mod kzmake_rca_user_v1 {
    tonic::include_proto!("kzmake.rca.user.v1");
}

pub struct Service<T>
where
    T: CreateUserPort,
{
    create: T,
}

impl<T> Service<T>
where
    T: CreateUserPort,
{
    pub fn new(create: T) -> Self {
        Self { create }
    }
}

#[tonic::async_trait]
impl<T> UserService for Service<T>
where
    T: CreateUserPort + Send + Sync + 'static,
{
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        println!("Got a request: {:?}", request);

        let input = CreateUserInputData {
            user_name: request.get_ref().name.to_string(),
        };

        match self.create.handle(input) {
            Ok(output) => Ok(Response::new(CreateResponse {
                user: Some(ProtoUser {
                    id: output.user_id.to_string(),
                    name: output.user_name.to_string(),
                }),
            })),
            Err(_) => Err(Status::internal("error")),
        }
    }
    async fn list(&self, _: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }
    async fn get(&self, _: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }
    async fn rename(&self, _: Request<RenameRequest>) -> Result<Response<RenameResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }
    async fn delete(&self, _: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }
}

impl<T> Service<T>
where
    T: CreateUserPort + Send + Sync + 'static,
{
    pub async fn serve(self, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        Server::builder()
            .add_service(UserServiceServer::new(self))
            .serve(addr)
            .await?;

        Ok(())
    }
}
