use developer_blog_business::models::example::Model;
use yew::{function_component, html, use_state, Callback, Html};

#[function_component(SimpleExample)]
pub fn simple_example() -> Html {
    html! {
        <div>
            <p>{"Hello there!"}</p>
        </div>
    }
}

#[function_component(SimpleStateExample)]
pub fn simple_state_example() -> Html {
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
