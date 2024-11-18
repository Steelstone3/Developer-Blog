use developer_blog_business::route::Route;
use style::global_style;
use views::view_switcher::switch_view;
use yew::prelude::*;
use yew_router::Switch;

mod style;
mod views;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <style>{global_style()}</style>
            <body>
                <Switch<Route> render={switch_view}/>
            </body>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
