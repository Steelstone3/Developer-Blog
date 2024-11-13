use developer_blog_business::models::example::Model;
use style::global_style;
use yew::prelude::*;

mod style;
mod views;

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

            // <routes>
            //     <route path={Path::Example.to_string()} element={""}/>
            // </routes>
            <button onclick={on_click}>{"+1"} </button>
            <p>{state.value}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
