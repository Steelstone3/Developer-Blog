use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <p>{"Hello there!"}</p>
        </div>
    }
}
