use developer_blog_business::route::Route;
use yew::{html, Html};

use super::{
    content::{
        blog::Blog,
        blogs::{Blogs, Example},
    },
    core::home::Home,
};

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
