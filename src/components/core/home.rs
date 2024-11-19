use yew::prelude::*;

use crate::components::core::navigation::Navigation;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <Navigation/>
            <p>{"hello world"}</p>
        </div>
    }
}
