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
            <body>
                <button onclick={on_click}>{"Calculate"} </button>
                <p>{state.value}</p>
                <h1>{"Hey"}</h1>
                <h2>{"Cringe"}</h2>
                <p>{"Cringe is cringe. You are cringe."}</p>
                <h3>{"More Boxing"}</h3>
            </body>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
