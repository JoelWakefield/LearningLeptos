#[path = "components/mod.rs"]
mod components;

use components::forms_inputs::{ControlledInput, Select, TextArea, UncontrolledInput};
use components::iteration::Iteration;
use components::progress_bar::ProgressBar;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <h1>"Learning Leptos"</h1>
        <div>
            <h2>"State Mangaement"</h2>
            <button
                on:click=move |_| {
                    *set_count.write() += 1;
                }
                class:red=move || count.get() % 2 == 1
            >
                "Count: "
                {count}
            </button>
            <br />
            <ProgressBar progress=count />
            <ProgressBar progress=Signal::derive(double_count) />
        </div>
        <br />
        <div>
            <h2>"Iteration"</h2>
            <Iteration />
        </div>
        <br />
        <div>
            <h2>"Forms and Inputs"</h2>
            <ControlledInput />
            <UncontrolledInput />
            <TextArea />
            <Select />
        </div>
    }
}
