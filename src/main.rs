use components::core::application::Application;

// web
mod components;
// server
mod database;
mod routes;
//shared
mod controllers;
mod models;

fn main() {
    // #[cfg(feature="web")]
    yew::Renderer::<Application>::new().render();
}
