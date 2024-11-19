use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Switch};
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
                <BrowserRouter>
                    <Switch<Route> render={switch}/>
                </BrowserRouter>
            </body>
        </div>
    }
}
