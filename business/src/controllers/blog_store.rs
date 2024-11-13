use crate::models::{
    blog_post::BlogPost, blog_store::BlogPostStore, id_blog_post::IdBlogPost,
    pagination::Pagination, update_blog_post::UpdateBlogPost,
};
use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

impl BlogPostStore {
    pub fn new_from_hashmap(store: HashMap<usize, IdBlogPost>) -> Self {
        let id = AtomicUsize::new(store.keys().max().map(|v| v + 1).unwrap_or(0));

        Self { id, store }
    }

    pub fn add_blog_post(&mut self, blog_post: BlogPost) -> IdBlogPost {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        let new_blog_post = IdBlogPost::new(id, blog_post);
        self.store.insert(id, new_blog_post.clone());

        new_blog_post
    }

    pub fn get_blog_post(&self, id: usize) -> Option<&IdBlogPost> {
        self.store.get(&id)
    }

    pub fn get_blog_posts(&self, pagination: Pagination) -> Vec<IdBlogPost> {
        self.store
            .values()
            .skip(pagination.offset.unwrap_or(usize::MIN))
            .take(pagination.limit.unwrap_or(usize::MAX))
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn update_blog_post(
        &mut self,
        id: &usize,
        update_blog_post: UpdateBlogPost,
    ) -> Option<&IdBlogPost> {
        if let Some(id_blog_post) = self.store.get_mut(id) {
            if let Some(title) = update_blog_post.title {
                id_blog_post.blog_post.title = title;
            }
            if let Some(author) = update_blog_post.author {
                id_blog_post.blog_post.author = author;
            }
            if let Some(description) = update_blog_post.description {
                id_blog_post.blog_post.description = description;
            }
            if let Some(contents) = update_blog_post.contents {
                id_blog_post.blog_post.contents = contents;
            }

            Some(id_blog_post)
        } else {
            None
        }
    }

    pub fn remove_blog_post(&mut self, id: usize) -> Option<IdBlogPost> {
        self.store.remove(&id)
    }

    // TODO add persistence
}
