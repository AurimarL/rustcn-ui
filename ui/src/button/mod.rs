//ui/button/mod.rs
use dioxus::prelude::*;

mod props;
pub use props::{ButtonProps, ButtonSize, ButtonVariants};

mod style;
use style::{get_size_class, get_variant_class, BASE_CLASS};

#[component]
pub fn Button(props: ButtonProps) -> Element {
    // Destructure the props for easier access
    let ButtonProps {
        class,
        variant,
        size,
        children,
        onclick,
    } = props;

    let variant_class = get_variant_class(&variant);
    let size_class = get_size_class(&size);

    // Combine the base, variant, and size classes with any additional classes provided
    let combined_class = format!("{} {} {} {}", BASE_CLASS, variant_class, size_class, class);

    rsx! {
        button {
            class: "{combined_class}",
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            {children}
        }
    }
}
