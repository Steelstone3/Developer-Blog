use crate::routes::route::Route;
use yew::{function_component, html, Callback, Html};
use yew_router::{hooks::use_navigator, prelude::Link};

#[function_component(Navigation)]
pub fn navigation() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Root));
        html! {
            <button {onclick}>{"click to go home"}</button>
        }
    };

    html! {
        <div>
            {go_home_button}
            <Link<Route> to={Route::Root}>{ "click here to go home" }</Link<Route>>
            <Link<Route> to={Route::Blogs}>{ "click here to go to blogs" }</Link<Route>>
        </div>
    }
}

