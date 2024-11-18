use style::global_style;
use views::profiles::{SimpleExample, SimpleStateExample};
use yew::prelude::*;

mod style;
mod views;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <style>{global_style()}</style>

            // <routes>
            //     <route path={Path::Example.to_string()} element={""}/>
            // </routes>
            <body>
                <SimpleExample/>
                <SimpleStateExample />
                <h1>{"Hey"}</h1>
                <h2>{"How Are You?"}</h2>
                <p>{"Learnt callbacks, hooks and components."}</p>
                <h3>{"More Coming Soon!"}</h3>
            </body>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
