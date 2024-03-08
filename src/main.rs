#![allow(non_snake_case)]
use dioxus::prelude::*;

mod ui;
use ui::button::{Button, ButtonSize, ButtonVariants};
use ui::input::Input;

use ui::template::{Template, TemplateVariants};

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
                size: ButtonSize::Default,
                text: "Press me",
            }
            Input{
                placeholder:"Escreva algo aqui"
            }
            Template{
                variant:TemplateVariants::Default
            }

        }
    }
}
