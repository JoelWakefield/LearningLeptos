use leptos::prelude::*;

/// Displays a progress bar.
#[component]
pub fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// The current value of the progress bar.
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
        <br/>
    }
}