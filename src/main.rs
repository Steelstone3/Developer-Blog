use components::core::application::Application;

// web
#[cfg(feature = "web")]
mod components;

// server
#[cfg(feature = "server")]
mod database;

#[cfg(feature = "server")]
mod routes;

//shared
mod controllers;
mod models;

fn main() {
    #[cfg(feature = "web")]
    yew::Renderer::<Application>::new().render();
}
