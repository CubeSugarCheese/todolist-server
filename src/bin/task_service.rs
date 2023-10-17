#![feature(impl_trait_in_assoc_type)]

use dotenv::dotenv;
use std::net::SocketAddr;
use todolist_server::rpc_server::db::{DATABASE, DB};
use todolist_server::rpc_server::handler::S;
use todolist_server::rpc_server::layer::LogLayer;

async fn entry() -> anyhow::Result<()> {
    dotenv()?;
    tracing_subscriber::fmt::init();
    let result = DB.set(DATABASE::new(&std::env::var("DATABASE_URL")?).await?);
    if let Err(e) = result {
        tracing::error!("init db failed with: {e}",)
    }

    let task_service_addr: SocketAddr = std::env::var("RPC_TASK_SERVICE_ADDR")
        .expect("RPC_TASK_SERVICE_ADDR muse be set")
        .parse()?;
    let task_service_addr = volo::net::Address::from(task_service_addr);
    volo_gen::task::TaskServiceServer::new(S)
        .layer(LogLayer)
        .run(task_service_addr)
        .await
        .unwrap();
    Ok(())
}

#[volo::main]
async fn main() {
    let result = entry().await;
    if let Err(e) = result {
        tracing::error!("start task rpc server failed with: {e}")
    }
}
