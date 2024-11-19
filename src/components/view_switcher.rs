use super::content::blog::Blog;
use crate::{
    components::content::blog::{Blogs, Example, Home},
    routes::route::Route,
};
use yew::{html, Html};

pub fn switch_view(route: Route) -> Html {
    let content = match route {
        Route::Root => html! {<Home/>},
        Route::Blogs => html! {<Blogs/>},
        Route::BlogId => html! {<Blog/>},
        Route::Example => html! {<Example/>},
        // Route::NotFound => html! { <Redirect<Route> to={Route::Home} /> },
    };

    html! {
        <div>
            <header route = {route.to_string()}> </header>
            {content}
            <footer> </footer>
        </div>
    }
}
