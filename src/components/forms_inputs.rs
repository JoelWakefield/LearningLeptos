use leptos::{ev::SubmitEvent, html, prelude::*};

#[component]
pub fn ControlledInput() -> impl IntoView {
    let (name, set_name) = signal("Name".to_string());
    let (fact, set_fact) = signal("Fact".to_string());
    let email = RwSignal::new("".to_string());
    let spam = RwSignal::new(true);

    view! {
        <h3>"Controlled Input"</h3>
        <input
            type="text"
            // adding :target gives us typed access to the element
            // that is the target of the event that fires
            on:input:target=move |ev| {
                set_fact.set(ev.target().value());
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=fact
        />
        <input type="text" bind:value=(name, set_name) />
        <input type="email" bind:value=email />
        <label>
            "Please, send me lots of spam email." <input type="checkbox" bind:checked=spam />
        </label>

        <p>"Face: " {fact}</p>
        <p>"Name: " {name}</p>
        <p>"Email: " {email}</p>
        <Show when=move || spam.get()>
            <p>"You'll receive cool bonus content!"</p>
        </Show>
    }
}

#[component]
pub fn UncontrolledInput() -> impl IntoView {
    let (name, set_name) = signal("Name".to_string());
    let input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name.set(value);
    };

    view! {
        <h3>"Controlled Input"</h3>
        // on_submit defined below
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element />
            <input type="submit" value="Submit" />
        </form>
        <p>"Name: " {name}</p>
    }
}

#[component]
pub fn TextArea() -> impl IntoView {
    let (value, set_value) = signal("TextArea".to_string());

    view! {
        <h3>"Text Area"</h3>
        <textarea
            prop:value=move || value.get()
            on:input:target=move |ev| set_value.set(ev.target().value())
        >

            {value.get_untracked()}
        </textarea>
    }
}

#[component]
pub fn Select() -> impl IntoView {
    let (value, set_value) = signal(0i32);

    view! {
        <h3>"Select"</h3>

        <select
            on:change:target=move |ev| {
                set_value.set(ev.target().value().parse().unwrap());
            }
            prop:value=move || value.get().to_string()
        >
            <option value="0">"0"</option>
            <option value="1">"1"</option>
            <option value="2">"2"</option>
        </select>
        // a button that will cycle through the options
        <button on:click=move |_| {
            set_value
                .update(|n| {
                    if *n == 2 {
                        *n = 0;
                    } else {
                        *n += 1;
                    }
                })
        }>"Next Option"</button>
    }
}
