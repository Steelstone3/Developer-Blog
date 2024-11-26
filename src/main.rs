use components::application::Application;
use leptos::*;
use mount::mount_to_body;

mod components;
mod models;
mod handlers;
mod controllers;
mod routes;

pub fn main() {
    mount_to_body(|| {
        view! {
            <Application initial_value=3 />
        }
    })
}
