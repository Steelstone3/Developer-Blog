use crate::routes::route::Route;
use yew::{function_component, html, Callback, Html};
use yew_router::hooks::use_navigator;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    let navigator = use_navigator().unwrap();

    let home = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Root));
        html! {
            <button {onclick}>{"Home"}</button>
        }
    };

    let blogs = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Blogs));
        html! {
            <button {onclick}>{"Blogs"}</button>
        }
    };

    html! {
        <div>
            {home}
            {blogs}
            // <Link<Route> to={Route::Root}>{ "click here to go home" }</Link<Route>>
            // <Link<Route> to={Route::Blogs}>{ "click here to go to blogs" }</Link<Route>>
        </div>
    }
}
