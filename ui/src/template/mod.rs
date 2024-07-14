//ui/template/mod.rs
use dioxus::prelude::*;

mod props;
pub use props::*;

mod style;
use style::{get_variant_class, BASE_CLASS};

pub fn Template(props: TemplateProps) -> Element {
    //Split props for easy access
    let TemplateProps { variant } = props;

    // Match the provided variant to its corresponding CSS class
    let variant_class = get_variant_class(&variant);

    // TODO: cn, tailwind-merge... must be implemented
    let combined_class = format!("{} {} ", BASE_CLASS, variant_class);

    //Render the Custom Component;
    rsx! {
        div {
            class: "{combined_class}",
            "Custom Component",
        }
    }
}
