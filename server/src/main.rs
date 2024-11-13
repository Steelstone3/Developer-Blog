use routes::router::run_server;

mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    run_server().await;
}
