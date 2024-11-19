use yew::prelude::*;

use crate::components::{
    content::blog::{Blog, Blogs, Example},
    core::style::global_style,
};

// #[cfg(feature="web")]
#[function_component(Application)]
pub fn application() -> Html {
    html! {
        <div>
            <style>{global_style()}</style>
            <body>
                <p>{"hello world"}</p>
                <Blog/>
                <Blogs/>
                <Example/>
                // <Switch<Route> render={switch_view}/>
            </body>
        </div>
    }
}
