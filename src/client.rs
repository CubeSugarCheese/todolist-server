use once_cell::sync::Lazy;
use std::net::SocketAddr;

pub mod model;
pub mod router;

static USER_SERVICE_CLIENT: Lazy<volo_gen::user::UserServiceClient> = Lazy::new(|| {
    let addr: SocketAddr = "[::1]:8080".parse().unwrap();
    volo_gen::user::UserServiceClientBuilder::new("todolist-server")
        .address(addr)
        .build()
});

static TASK_SERVICE_CLIENT: Lazy<volo_gen::task::TaskServiceClient> = Lazy::new(|| {
    let addr: SocketAddr = "[::1]:8081".parse().unwrap();
    volo_gen::task::TaskServiceClientBuilder::new("todolist-server")
        .address(addr)
        .build()
});
