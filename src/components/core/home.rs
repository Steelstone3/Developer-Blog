use yew::prelude::*;

// #[cfg(feature="web")]
#[function_component(Application)]
pub fn application() -> Html {
    html! {
        <div>
            // <style>{global_style()}</style>
            <body>
                <p>{"hello world"}</p>
                // <Switch<Route> render={switch_view}/>
            </body>
        </div>
    }
}
