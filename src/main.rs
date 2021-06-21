use anyhow::Result;

pub mod domain;
pub mod infrastructure;
pub mod interface;
pub mod usecase;

use crate::infrastructure::grpc::Service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    println!("service listening on {}", addr);

    Service::default().serve(addr).await?;

    Ok(())
}
