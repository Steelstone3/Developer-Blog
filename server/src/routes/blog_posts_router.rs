use crate::handlers::blog_posts::{
    add_blog_post, delete_blog_post, get_blog_post, get_blog_posts, update_blog_post,
};
use axum::{routing::get, Router};
use developer_blog_business::path::Path;

use super::router::Database;

pub fn build_blog_posts_router() -> Router {
    let database = Database::default();

    Router::new()
        .route(
            Path::Blogs.to_string().as_str(),
            get(get_blog_posts).post(add_blog_post),
        )
        .route(
            Path::BlogId.to_string().as_str(),
            get(get_blog_post)
                .delete(delete_blog_post)
                .patch(update_blog_post),
        )
        // TODO add persistence
        // .route("/profile/persist", post(persist))
        .with_state(database)
}
