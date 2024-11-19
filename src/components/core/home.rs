use yew::prelude::*;
use yew_router::Switch;

use crate::{
    components::{core::style::global_style, switch::switch},
    routes::route::Route,
};

// #[cfg(feature="web")]
#[function_component(Application)]
pub fn application() -> Html {
    html! {
        <div>
            <style>{global_style()}</style>
            <body>
                // <Blog/>
                // <Blogs/>
                // <Example/>

                <Switch<Route> render={switch}/>
            </body>
        </div>
    }
}
