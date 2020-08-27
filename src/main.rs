use style::global_style;
use yew::prelude::*;

mod client;
mod server;
mod style;

struct Model {
    value: i32,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model { value: 0 });

    let on_click = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1,
            });
        })
    };

    html! {
        <div>
        <style>{global_style()}</style>
            <style>

            </style>

            <button onclick={on_click}>{"+1"} </button>
            <p>{state.value}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
