use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UpdateBlogPost {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub contents: Option<String>,
}
