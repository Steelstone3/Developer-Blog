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
        Route::BlogId => html! {<Blog/>},
        Route::Example => html! {<Example/>},
        Route::NotFound => {
            html! {<p>{"Ooh.  I actually don't know where that page went.  Maybe start at <a href='https://ferrywizard.com'>home</a> and try again?"}</p>}
        }
    };

    html! {
        <div>
            <header route = {route.to_string()}> </header>
            {content}
            <footer> </footer>
        </div>
    }
}
