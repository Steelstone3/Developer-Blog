use yew::{function_component, html, use_state, Callback, Html};

#[function_component(Example)]
pub fn example() -> Html {
    let state = use_state(|| Model { value: 0 });

    let increment = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1,
            });
        })
    };

    html! {
        <div>
            <button onclick={increment}>{"+1 To Value"} </button>
            <p>{state.value}</p>
        </div>
    }
}

#[function_component(Blogs)]
pub fn blogs() -> Html {
    html! {
        <div>
            <h1>{"This is where all the blogs will go"}</h1>
        </div>
    }
}

#[function_component(Blog)]
pub fn blog() -> Html {
    html! {
        <div>
            <h1>{"This is where specific blogs will be read"}</h1>
        </div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <p>{"hello world"}</p>
        </div>
    }
}

// TODO Remove example model
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Model {
    pub value: i32,
}
