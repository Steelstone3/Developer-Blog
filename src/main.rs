use components::application::Application;
use leptos::*;
use mount::mount_to_body;

mod components;

pub fn main() {
    mount_to_body(|| {
        view! {
            <Application initial_value=3 />
        }
    })
}
