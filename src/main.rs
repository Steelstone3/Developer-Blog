use components::core::home::Home;

mod components;
mod database;
mod routes;
mod models;

fn main() {
    // #[cfg(feature="web")]
    yew::Renderer::<Home>::new().render();
}
