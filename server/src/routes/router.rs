use axum::{
    extract::{Path, Query},
    http::Method,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir};

pub async fn run_server() {
    let router = build_router();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

fn build_router() -> Router {
    let cors = CorsLayer::new().allow_methods([Method::GET, Method::POST, Method::PATCH]);

    Router::new()
        .merge(build_hello_router())
        .fallback_service(build_static_router())
        .layer(cors)
}

// Routers

fn build_static_router() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn build_hello_router() -> Router {
    Router::new()
        .route("/hello", get(hello_world))
        .route("/hello_2", get(hello_world_2))
        .route("/hello_3/:name", get(hello_world_3))
}

// Models

#[derive(Deserialize)]
pub struct HelloParameters {
    name: Option<String>,
}

// Controllers

pub async fn hello_world() -> impl IntoResponse {
    println!("Something Something Not Important");

    Html("Hello World!")
}

// query extractor
// e.g /hello_2?name=Bob
pub async fn hello_world_2(Query(params): Query<HelloParameters>) -> impl IntoResponse {
    println!("Something Something Not Important 2");

    let name = params.name.unwrap_or("No Name!".to_string());

    Html(format!("Hello World! {name}"))
}

// path extractor
// e.g /hello_3/Derek
pub async fn hello_world_3(Path(name): Path<String>) -> impl IntoResponse {
    println!("Something Something Not Important 3");

    Html(format!("Hello World! {name}"))
}
