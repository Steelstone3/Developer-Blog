use leptos::html::*;
use leptos::*;
use mount::mount_to_body;
use prelude::{OnAttribute, Update};
use reactive::signal::signal;

// The #[component] macro marks a function as a reusable component
// Components are the building blocks of your user interface
// They define a reusable unit of behavior
#[component]
fn App(initial_value: u32) -> impl IntoView {
    let (value, set_value) = signal(initial_value);
    let clear = move |_| set_value.update(|value| *value = 0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    div().child((
        button().on(ev::click, clear).child("Clear"),
        button().on(ev::click, decrement).child("-1"),
        span().child(("Value: ", value, "!")),
        button().on(ev::click, increment).child("+1"),
    ))
}

pub fn main() {
    mount_to_body(|| {
        view! {
            <App initial_value=3 />
        }
    })
}
