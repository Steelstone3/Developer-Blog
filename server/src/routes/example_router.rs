use axum::{routing::get, Router};
use developer_blog_business::path::Path;

use crate::handlers::example::update_example;

pub fn build_example_router() -> Router {
    Router::new().route(Path::Example.to_string().as_str(), get(update_example))
}
