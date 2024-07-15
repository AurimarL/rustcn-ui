//ui/button/mod.rs
use dioxus::prelude::*;

mod props;
pub use props::{ButtonProps, ButtonSize, ButtonVariants};

mod style;
use style::{get_size_class, get_variant_class, BASE_CLASS};

pub fn Button(props: ButtonProps) -> Element {
    // Destructure the props for easier access
    let ButtonProps {
        class,
        variant,
        size,
        text,
    } = props;

    let variant_class = get_variant_class(&variant);
    let size_class = get_size_class(&size);

    // Combine the base, variant, and size classes with any additional classes provided
    let combined_class = format!("{} {} {} {}", BASE_CLASS, variant_class, size_class, class);

    rsx! {
        button {  class: "{combined_class}" ,"{text}" }
    }
}
