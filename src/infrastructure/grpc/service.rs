use anyhow::Result;
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

pub mod kzmake_rca_user_v1 {
    tonic::include_proto!("kzmake.rca.user.v1");
}

use kzmake_rca_user_v1::user_service_server::{UserService, UserServiceServer};
use kzmake_rca_user_v1::User;
use kzmake_rca_user_v1::{CreateRequest, CreateResponse};
use kzmake_rca_user_v1::{DeleteRequest, DeleteResponse};
use kzmake_rca_user_v1::{GetRequest, GetResponse};
use kzmake_rca_user_v1::{ListRequest, ListResponse};
use kzmake_rca_user_v1::{RenameRequest, RenameResponse};

use crate::domain::repository::HaveIdRepository;
use crate::domain::repository::HaveUserRepository;
use crate::infrastructure::id::UlidRepository;
use crate::infrastructure::state::MemoryUserRepository;
use crate::usecase::interactor::CreateUserInterractor;
use crate::usecase::port::{CreateUserInputData, CreateUserPort, HaveCreateUserPort};

#[derive(Default)]
pub struct Service {}

// Inject MemoryUserRepository
impl MemoryUserRepository for Service {}
impl HaveUserRepository for Service {
    type UserRepository = Self;
    fn provide_user_repository(&self) -> &Self::UserRepository {
        &self
    }
}

// Inject UlidRepository
impl UlidRepository for Service {}
impl HaveIdRepository for Service {
    type IdRepository = Self;
    fn provide_id_repository(&self) -> &Self::IdRepository {
        &self
    }
}

// Inject CreateUserInterractor
impl CreateUserInterractor for Service {}
impl HaveCreateUserPort for Service {
    type CreateUserPort = Self;
    fn provide_create_user_port(&self) -> &Self::CreateUserPort {
        &self
    }
}

#[tonic::async_trait]
impl UserService for Service {
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        println!("Got a request: {:?}", request);

        let input = CreateUserInputData {
            user_name: request.get_ref().name.to_string(),
        };

        match self.provide_create_user_port().handle(input) {
            Ok(output) => Ok(Response::new(CreateResponse {
                user: Some(User {
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

impl Service {
    pub async fn serve(self, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        Server::builder()
            .add_service(UserServiceServer::new(self))
            .serve(addr)
            .await?;

        Ok(())
    }
}
