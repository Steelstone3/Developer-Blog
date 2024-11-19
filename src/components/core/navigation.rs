use yew::{function_component, html, Html};
use yew_router::prelude::Link;
use crate::routes::route::Route;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <div>
            <Link<Route> to={Route::Root}>{ "click here to go home" }</Link<Route>>
        </div>
    }
}
