//ui/button/props.rs
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    #[props(default)]
    pub class: String,

    #[props(default)]
    pub variant: ButtonVariants,

    #[props(default)]
    pub size: ButtonSize,

    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    #[props(default)]
    pub children: Element,

}

#[derive(PartialEq, Clone)]
pub enum ButtonVariants {
    Default,
    _Secondary,
    _Destructive,
    _Outline,
    _Ghost,
    _Link,
    // Additional variants can be added here.
}

impl Default for ButtonVariants {
    fn default() -> Self {
        ButtonVariants::Default
    }
}

#[derive(PartialEq, Clone)]
pub enum ButtonSize {
    Default,
    _SM,
    _LG,
    _ICON,
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Default
    }
}
