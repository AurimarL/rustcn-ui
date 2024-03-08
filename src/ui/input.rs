//input.rs
use dioxus::prelude::*;

/// Represents properties that can be passed to the `Input` component.
#[derive(PartialEq, Props, Clone)]
pub struct InputProps {
    /// An optional CSS class to be added to the input for additional styling.
    #[props(optional)]
    class: String,
    /// Specifies the visual variant of the input, which defines its appearance.
    /// Defaults to `InputVariants::Default`.
    #[props(default)]
    variant: InputVariants,
    /// An optional property to specify the size of the input.
    /// Defaults to `InputSize::Default`.
    #[props(optional)]
    size: InputSize,
    /// Placeholder text for the input. If not provided, a default placeholder will be shown.
    #[props(default = "Type something...".to_string())]
    placeholder: String,
    // Callback for handling changes to the input.
    // #[props(optional)]
    // onchange: UseCallback<T>,
}

/// Defines the possible variants for an input, influencing its styling.
#[derive(PartialEq, Clone)]
pub enum InputVariants {
    Default,
    _Outline,
    _Underline,
    _Flushed,
    _Filled,
    // Additional variants can be added here.
}

/// Provides a default variant for `InputVariants`.
impl Default for InputVariants {
    fn default() -> Self {
        InputVariants::Default
    }
}

/// Defines the possible sizes for an input.
#[derive(PartialEq, Clone)]
pub enum InputSize {
    Default,
    _SM,
    _LG,
}

/// Provides a default size for `InputSize`.
impl Default for InputSize {
    fn default() -> Self {
        InputSize::Default
    }
}

pub fn Input(props: InputProps) -> Element {
    let InputProps {
        class,
        variant,
        size,
        placeholder,
        // onchange,
    } = props;

    // Match the provided variant to its corresponding CSS class
    let variant_class = match variant {
        InputVariants::Default => VARIANT_DEFAULT,
        InputVariants::_Outline => VARIANT_OUTLINE,
        InputVariants::_Underline => VARIANT_UNDERLINE,
        InputVariants::_Flushed => VARIANT_FLUSHED,
        InputVariants::_Filled => VARIANT_FILLED,
        // Handle additional variants here
    };

    // Match the provided size to its corresponding CSS class
    let size_class = match size {
        InputSize::_SM => SIZE_SM,
        InputSize::_LG => SIZE_LG,
        _ => SIZE_DEFAULT,
    };

    // Combine the base, variant, and size classes with any additional classes provided
    let combined_class = format!("{} {} {} {}", BASE_CLASS, variant_class, size_class, class);

    // Render the input element with the combined classes, placeholder and onchange event
    rsx! {
        input {
            class: "{combined_class}",
            placeholder: "{placeholder}",
            // oninput: move |event| onchange.emit(event),
        }
    }
}

// Constants defining base, variant, and size classes for input styling
const BASE_CLASS: &str = "flex h-9 rounded-md border border-input bg-transparent text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";

const VARIANT_DEFAULT: &str = "bg-white";
const VARIANT_OUTLINE: &str = "border-2";
const VARIANT_UNDERLINE: &str = "border-b-2 border-t-0 border-l-0 border-r-0";
const VARIANT_FLUSHED: &str = "border-b-2";
const VARIANT_FILLED: &str = "bg-gray-200";

const SIZE_DEFAULT: &str = "px-3 py-1";
const SIZE_SM: &str = "text-xs px-2 py-0.5";
const SIZE_LG: &str = "text-lg px-4 py-2";
