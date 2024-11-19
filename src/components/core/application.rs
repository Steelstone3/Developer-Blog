use crate::{
    components::{core::style::global_style, switch::switch},
    routes::route::Route,
};
use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Switch};

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
