use axum::{routing::get, Router};
use developer_blog_business::route::Route;

use crate::handlers::example::update_example;

pub fn build_example_router() -> Router {
    Router::new().route(Route::Example.to_string().as_str(), get(update_example))
}
