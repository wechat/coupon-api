use axum::{serve, Error};
use common::log;
use dotenv::{dotenv, var};
use routers::init::router;
use std::{net::SocketAddr, str::FromStr};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 加载环境变量
    dotenv().ok();
    // 启动日志系统
    log::start_logs();
    // 加载路由
    let app = router();
    // 启动服务
    let host_str = var("HOST").expect("HOST must be set");
    let port_str = var("PORT").expect("PORT must be set");
    println!("Starting server at http://{}:{}", host_str, port_str);
    let addr =
        SocketAddr::from_str(&format!("{}:{}", host_str, port_str)).expect("SocketAddr fail");
    let listener = TcpListener::bind(&addr).await.expect("listener fail");
    serve(listener, app).await.expect("serve start fail");
    println!("Service started successfully at http://{}", addr);
    Ok(())
}
