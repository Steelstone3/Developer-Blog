use super::blog_post::BlogPost;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct IdBlogPost {
    pub id: usize,
    #[serde(flatten)]
    pub blog_post: BlogPost,
}

impl IdBlogPost {
    pub fn new(id: usize, blog_post: BlogPost) -> IdBlogPost {
        IdBlogPost { id, blog_post }
    }
}
