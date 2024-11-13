use axum::{
    http::Method,
    Router,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use super::{profiles_router::build_profiles_router, static_router::build_static_router};

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
        .merge(build_profiles_router())
        .fallback_service(build_static_router())
        .layer(cors)
}

// Routers

// #[allow(dead_code)]
// fn build_hello_router() -> Router {
//     Router::new()
//         .route("/hello", get(hello_world))
//         .route("/hello_2", get(hello_world_2))
//         .route("/hello_3/:name", get(hello_world_3))
// }

// Models

// #[derive(Deserialize)]
// pub struct HelloParameters {
//     name: Option<String>,
// }

// Controllers

// pub async fn hello_world() -> impl IntoResponse {
//     println!("Something Something Not Important");

//     Html("Hello World!")
// }

// query extractor
// e.g /hello_2?name=Bob
// pub async fn hello_world_2(Query(params): Query<HelloParameters>) -> impl IntoResponse {
//     println!("Something Something Not Important 2");

//     let name = params.name.unwrap_or("No Name!".to_string());

//     Html(format!("Hello World! {name}"))
// }

// path extractor
// e.g /hello_3/Derek
// pub async fn hello_world_3(Path(name): Path<String>) -> impl IntoResponse {
//     println!("Something Something Not Important 3");

//     Html(format!("Hello World! {name}"))
// }
