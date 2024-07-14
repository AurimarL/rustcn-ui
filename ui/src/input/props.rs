//ui/input/props.rs
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct InputProps {
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub variant: InputVariants,
    #[props(default)]
    pub size: InputSize,
    #[props(default = "Type something...".to_string())]
    pub placeholder: String,
}

#[derive(PartialEq, Clone)]
pub enum InputVariants {
    Default,
    _Outline,
    _Underline,
    _Flushed,
    _Filled,
}

impl Default for InputVariants {
    fn default() -> Self {
        InputVariants::Default
    }
}

#[derive(PartialEq, Clone)]
pub enum InputSize {
    Default,
    _SM,
    _LG,
}

impl Default for InputSize {
    fn default() -> Self {
        InputSize::Default
    }
}
