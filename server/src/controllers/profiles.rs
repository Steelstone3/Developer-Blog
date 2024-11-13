use axum::{
    extract::Query,
    response::{Html, IntoResponse},
};
use developer_blog_shared::models::profile::Profile;

pub async fn get_profile(Query(params): Query<Profile>) -> impl IntoResponse {
    println!("profile");

    let name = params.name.unwrap_or("No name".to_string());
    let description = params.description.unwrap_or("No description".to_string());

    Html(format!("{name}\n{description}"))
}
