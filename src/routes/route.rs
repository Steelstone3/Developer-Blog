use yew_router::Routable;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/blogs")]
    Blogs,
    #[at("/blog/:id")]
    // BlogId { id: String },
    BlogId,
    #[at("/example")]
    Example,
    #[not_found]
    #[at("/404")]
    NotFound,
}
