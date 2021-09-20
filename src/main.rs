pub mod domain;
pub mod infrastructure;
pub mod interface;
pub mod usecase;

use crate::domain::entity::User;
use crate::infrastructure::grpc::Service;
use crate::infrastructure::id::UlidRepository;
use crate::infrastructure::state::MemoryRepository;
use crate::usecase::interactor::CreateUserInteractor;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    println!("service listening on {}", addr);

    let id_repository = UlidRepository::new();
    let user_repository = MemoryRepository::<User>::new();
    let create_port = CreateUserInteractor::new(id_repository, user_repository);

    Service::new(create_port).serve(addr).await?;

    Ok(())
}
