use std::fmt::{Display, Formatter, Result};

use yew_router::Routable;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/blogs")]
    Blogs,
    #[at("/blog/:id")]
    BlogId,
    #[at("/example")]
    Example,
}

impl Display for Route {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Route::Root => write!(formatter, "/"),
            Route::Blogs => write!(formatter, "/blogs"),
            Route::BlogId => write!(formatter, "/blog/:id"),
            Route::Example => write!(formatter, "/example"),
            // Route::NotFound => write!(formatter, "/"),
        }
    }
}
