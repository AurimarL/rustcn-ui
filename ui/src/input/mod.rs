//ui/input/mod.rs
use dioxus::prelude::*;

mod props;
pub use props::InputProps;

mod style;
pub use style::{get_size_class, get_variant_class, BASE_CLASS};

pub fn Input(props: InputProps) -> Element {
    let InputProps {
        class,
        variant,
        size,
        placeholder,
        // onchange,
    } = props;

    // Match the provided variant to its corresponding CSS class
    let variant_class = get_variant_class(&variant);

    // Match the provided size to its corresponding CSS class
    let size_class = get_size_class(&size);

    let combined_class = format!("{} {} {} {}", BASE_CLASS, variant_class, size_class, class);

    rsx! {
        input {
            class: "{combined_class}",
            placeholder: "{placeholder}",
        }
    }
}
