use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use developer_blog_business::models::{
    pagination::Pagination, profile::Profile, update_profile::UpdateProfile,
};

use crate::routes::router::Database;

pub async fn get_profiles(
    pagination: Option<Query<Pagination>>,
    State(database): State<Database>,
) -> impl IntoResponse {
    let profile_store = database.read().await;
    let Query(pagination) = pagination.unwrap_or_default();

    Json(profile_store.get_profiles(pagination))
}

pub async fn get_profile(
    Path(id): Path<usize>,
    State(database): State<Database>,
) -> impl IntoResponse {
    let profile_store = database.read().await;

    if let Some(profile) = profile_store.get_profile(id) {
        Json(profile).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Not found").into_response()
    }
}

pub async fn add_profile(
    State(database): State<Database>,
    Json(profile): Json<Profile>,
) -> impl IntoResponse {
    let mut profile_store = database.write().await;
    let id_profile = profile_store.add_profile(profile);

    (StatusCode::CREATED, Json(id_profile))
}

pub async fn delete_profile(
    Path(id): Path<usize>,
    State(database): State<Database>,
) -> impl IntoResponse {
    match database.write().await.remove_profile(id).is_some() {
        true => StatusCode::NO_CONTENT,
        false => StatusCode::NOT_FOUND,
    }
}

pub async fn update_profile(
    Path(id): Path<usize>,
    State(database): State<Database>,
    Json(update_profile): Json<UpdateProfile>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut profile_store = database.write().await;
    let res = profile_store.update_profile(&id, update_profile);
    match res {
        Some(todo) => Ok(Json(todo.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
