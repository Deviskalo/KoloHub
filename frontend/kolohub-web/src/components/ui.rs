use dioxus::prelude::*;

#[component]
pub fn FormGroup(label: String, children: Element) -> Element {
    rsx! {
        div { class: "kh-form-group",
            label { class: "kh-form-label", "{label}" }
            {children}
        }
    }
}

#[component]
pub fn Input(
    input_type: Option<String>,
    placeholder: Option<String>,
    value: Option<String>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let input_type = input_type.unwrap_or_else(|| "text".to_string());
    let placeholder = placeholder.unwrap_or_default();

    rsx! {
        input {
            class: "kh-input",
            r#type: "{input_type}",
            placeholder: "{placeholder}",
            oninput: move |e| {
                if let Some(h) = oninput {
                    h.call(e);
                }
            },
        }
    }
}

#[component]
pub fn TextArea(
    placeholder: Option<String>,
    rows: Option<u32>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let placeholder = placeholder.unwrap_or_default();
    let rows = rows.unwrap_or(4);

    rsx! {
        textarea {
            class: "kh-input",
            style: "min-height: 120px; resize: vertical;",
            placeholder: "{placeholder}",
            rows: "{rows}",
            oninput: move |e| {
                if let Some(h) = oninput {
                    h.call(e);
                }
            },
        }
    }
}

#[component]
pub fn Select(options: Vec<String>, onchange: Option<EventHandler<FormEvent>>) -> Element {
    rsx! {
        select {
            class: "kh-input",
            style: "appearance: none; background-image: url('data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%2224%22%20height%3D%2224%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22none%22%20stroke%3D%22currentColor%22%20stroke-width%3D%222%22%20stroke-linecap%3D%22round%22%20stroke-linejoin%3D%22round%22%3E%3Cpolyline%20points%3D%226%209%2012%2015%2018%209%22%3E%3C%2Fpolyline%3E%3C%2Fsvg%3E'); background-repeat: no-repeat; background-position: right 0.75rem center; background-size: 1rem;",
            onchange: move |e| {
                if let Some(h) = onchange {
                    h.call(e);
                }
            },
            for opt in options {
                option { value: "{opt}", "{opt}" }
            }
        }
    }
}

#[component]
pub fn Button(
    children: Element,
    onclick: Option<EventHandler<MouseEvent>>,
    variant: Option<String>, // "brand", "outline", "accent"
) -> Element {
    let variant_class = match variant.as_deref() {
        Some("outline") => "kh-btn-outline",
        Some("accent") => "kh-btn-accent",
        _ => "kh-btn-brand",
    };

    rsx! {
        button {
            class: "{variant_class}",
            onclick: move |e| {
                if let Some(h) = onclick {
                    h.call(e);
                }
            },
            {children}
        }
    }
}
