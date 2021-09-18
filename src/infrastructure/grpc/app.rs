use anyhow::Result;
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
use crate::domain::repository::HaveRepository;
use crate::infrastructure::id::UlidRepository;
use crate::infrastructure::state::MemoryUserRepository;
use crate::usecase::interactor::CreateUserInterractor;
use crate::usecase::port::{CreateUserInputData, CreateUserPort, HaveCreateUserPort};

#[derive(Default)]
pub struct App {}

impl MemoryUserRepository for App {}
impl HaveRepository<User> for App {
    type Repository = Self;
    fn provide_repository(&self) -> &Self::Repository {
        &self
    }
}

impl UlidRepository for App {}
impl HaveIdRepository for App {
    type IdRepository = Self;
    fn provide_id_repository(&self) -> &Self::IdRepository {
        &self
    }
}

impl Interractor<CreateUser> for App {}
impl HavePort<CreateUser> for App {
    type Port = Self;
    fn provide_port(&self) -> &Self::Port {
        &self
    }
}

#[tonic::async_trait]
impl UserService for App {
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        println!("Got a request: {:?}", request.get_ref());

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

impl App {
    pub async fn run(address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let addr = address.parse()?;
        let app = App::default();

        Server::builder()
            .add_service(UserServiceServer::new(app))
            .serve(addr)
            .await?;

        Ok(())
    }
}
