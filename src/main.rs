// main.rs
#![allow(non_snake_case)]
use dioxus::prelude::*;

mod ui;
use ui::button::{Button, ButtonSize, ButtonVariants};
use ui::input::{Input, InputSize, InputVariants};

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2 justify-center items-center h-screen",
            Button {
                class: "custom-button-class",
                variant: ButtonVariants::Default,
                size: ButtonSize::_ICON,
                text: "Press me",
            }
            Input {
                class: "custom-input-class",
                variant: InputVariants::_Filled,
                size: InputSize::Default,
                placeholder: "Enter text here",
            }
        }
    }
}
