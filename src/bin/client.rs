use axum::middleware;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use dotenv::dotenv;
use std::net::SocketAddr;
use todolist_server::client::router::jwt::auth_middleware;
use todolist_server::client::router::task::{
    create_task, delete_task, get_multiple_task, get_task, update_task_status,
};
use todolist_server::client::router::user::{login, register};

async fn entry() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/task/create", post(create_task))
        .route("/task/set_status", patch(update_task_status))
        .route("/task/delete", delete(delete_task))
        .route("/task/get", get(get_task))
        .route("/task/get-multiple", get(get_multiple_task))
        .route_layer(middleware::from_fn(auth_middleware)) // 中间件只会应用到已经存在的 route 上，因此在 layer 之后注册的路由不受保护
        .route("/user/register", post(register))
        .route("/user/login", post(login));

    let addr: SocketAddr = std::env::var("HTTP_API_ADDR").expect("HTTP_API_ADDR muse be set").parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[volo::main]
async fn main() {
    let result = entry().await;
    if let Err(e) = result {
        tracing::error!("run client failed with: {e}")
    }
}
