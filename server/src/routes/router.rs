use super::{
    blog_posts_router::build_blog_posts_router, example_router::build_example_router,
    static_router::build_static_router,
};
use axum::{http::Method, Router};
use developer_blog_business::models::blog_store::BlogPostStore;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::cors::CorsLayer;

pub type Database = Arc<RwLock<BlogPostStore>>;

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
        .merge(build_blog_posts_router())
        .merge(build_example_router())
        .fallback_service(build_static_router())
        .layer(cors)
}
