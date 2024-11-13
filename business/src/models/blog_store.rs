use std::{collections::HashMap, sync::atomic::AtomicUsize};

use super::id_blog_post::IdBlogPost;

#[derive(Default)]
pub struct BlogPostStore {
    pub id: AtomicUsize,
    pub store: HashMap<usize, IdBlogPost>,
}
