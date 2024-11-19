use crate::routes::route::Route;
use yew::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <div>
            <Link<Route> to={Route::Root}>{ "click here to go home" }</Link<Route>>
            <Link<Route> to={Route::Blogs}>{ "click here to go to blogs" }</Link<Route>>
        </div>
    }
}
