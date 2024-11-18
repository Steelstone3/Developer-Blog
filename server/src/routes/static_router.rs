use axum::{routing::get_service, Router};
use developer_blog_business::route::Route;
use tower_http::services::ServeDir;

pub fn build_static_router() -> Router {
    Router::new().nest_service(
        Route::Root.to_string().as_str(),
        get_service(ServeDir::new(Route::Root.to_string().as_str())),
    )
}
