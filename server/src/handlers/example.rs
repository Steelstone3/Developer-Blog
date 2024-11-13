use axum::{
    extract::Query,
    response::{Html, IntoResponse},
};
use developer_blog_business::models::example::Model;

pub async fn update_example(Query(params): Query<Model>) -> impl IntoResponse {
    println!("update_example");

    let value = params.value;

    Html(format!("{value}"))
}
