use anyhow::Result;

pub mod domain;
pub mod infrastructure;
pub mod interface;
pub mod usecase;

use crate::infrastructure::grpc::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:50051";

    println!("service listening on {}", address);

    App::run(address).await?;

    Ok(())
}
