use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use developer_blog_business::models::{
    blog_post::BlogPost, pagination::Pagination, update_blog_post::UpdateBlogPost,
};

use crate::routes::router::Database;

pub async fn get_blog_posts(
    pagination: Option<Query<Pagination>>,
    State(database): State<Database>,
) -> impl IntoResponse {
    let blog_post_store = database.read().await;
    let Query(pagination) = pagination.unwrap_or_default();

    Json(blog_post_store.get_blog_posts(pagination))
}

pub async fn get_blog_post(
    Path(id): Path<usize>,
    State(database): State<Database>,
) -> impl IntoResponse {
    let blog_post_store = database.read().await;

    if let Some(blog_post) = blog_post_store.get_blog_post(id) {
        Json(blog_post).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Not found").into_response()
    }
}

pub async fn add_blog_post(
    State(database): State<Database>,
    Json(blog_post): Json<BlogPost>,
) -> impl IntoResponse {
    let mut blog_post_store = database.write().await;
    let id_blog_post = blog_post_store.add_blog_post(blog_post);

    (StatusCode::CREATED, Json(id_blog_post))
}

pub async fn delete_blog_post(
    Path(id): Path<usize>,
    State(database): State<Database>,
) -> impl IntoResponse {
    match database.write().await.remove_blog_post(id).is_some() {
        true => StatusCode::NO_CONTENT,
        false => StatusCode::NOT_FOUND,
    }
}

pub async fn update_blog_post(
    Path(id): Path<usize>,
    State(database): State<Database>,
    Json(update_blog_post): Json<UpdateBlogPost>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut blog_post_store = database.write().await;
    let res = blog_post_store.update_blog_post(&id, update_blog_post);
    match res {
        Some(id_blog_post) => Ok(Json(id_blog_post.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
