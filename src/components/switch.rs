use super::content::blog::Blog;
use crate::{
    components::content::blog::{Blogs, Example, Home},
    routes::route::Route,
};
use yew::{html, Html};

pub fn switch(route: Route) -> Html {
    let content = match route {
        Route::Root => html! {<Home/>},
        Route::Blogs => html! {<Blogs/>},
        // Route::BlogId { id } => html! {<Blog/>},
        Route::BlogId => html! {<Blog/>},
        Route::Example => html! {<Example/>},
        Route::NotFound => {
            html! {<p>{"404 Page Not Found"}</p>}
        }
    };

    html! {
        <div>
            <header/>
            {content}
            <footer/>
        </div>
    }
}
