use leptos::*;

/// A simple counter component.
/// 
/// You can use doc comments like this to document your component.
#[component]
pub fn SimpleCounter(
    cx: Scope,
    initial_value: i32,
    step: i32
) -> impl IntoView {
    let (value, set_value) = create_signal(cx, initial_value);

    view! { cx,
        <div>
            <button on:click=move |_| set_value(0)>"Clear"</button>
            <button on:click=move |_| set_value.update(|value| *value -= step)>"-1"</button>
            <span>"Value: " {move || value().to_string()} "!"</span>
            <button on:click=move |_| set_value.update(|value| *value += step)>"+1"</button>
        </div>
    }
}