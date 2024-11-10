use routes::router::run_server;

mod controllers;
mod routes;

#[tokio::main]
async fn main() {
    run_server().await;
}
