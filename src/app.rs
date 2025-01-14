#[path = "components/progress_bar.rs"] mod progress_bar;

use progress_bar::ProgressBar;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button 
            on:click=move |_| { *set_count.write() += 1; }
            class:red=move || count.get() % 2 == 1
        >
            "Count: " {count}
        </button>
        <ProgressBar progress=count />
        <ProgressBar progress=Signal::derive(double_count) />
    }
}