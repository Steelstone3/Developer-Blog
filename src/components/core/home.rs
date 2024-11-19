use crate::components::core::navigation::Navigation;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <Navigation/>
            <p>{"hello world"}</p>
        </div>
    }
}
