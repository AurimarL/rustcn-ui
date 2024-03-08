//button.rs
// Import Dioxus prelude for convenience
use dioxus::prelude::*;

/// Represents properties that can be passed to the `Button` component.
#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    /// An optional CSS class to be added to the button for additional styling.
    #[props(optional)]
    class: String,
    /// Specifies the visual variant of the button, which defines its appearance.
    /// Defaults to `ButtonVariants::Default`.
    #[props(default)]
    variant: ButtonVariants,
    /// An optional property to specify the size of the button.
    /// Defaults to `ButtonSize::Default`.
    #[props(optional)]
    size: ButtonSize,
    /// Text content of the button. If not provided, the button will be empty.
    #[props(default = "".to_string())]
    text: String,
}

/// Defines the possible variants for a button, influencing its styling.
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

/// Provides a default variant for `ButtonVariants`.
impl Default for ButtonVariants {
    fn default() -> Self {
        ButtonVariants::Default
    }
}

/// Defines the possible sizes for a button.
#[derive(PartialEq, Clone)]
pub enum ButtonSize {
    Default,
    _SM,
    _LG,
    _ICON,
}

/// Provides a default size for `ButtonSize`.
impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Default
    }
}

/// Renders a button with customizable properties such as variant, size, and additional classes.
///
/// ## Examples
///
/// ```rust
/// # use dioxus::prelude::*;
/// # use dioxus::events::MouseEvent;
/// # let onclick = |_: MouseEvent| {};
///
/// rsx! {
///     Button {
///         class: "additional-class",
///         variant: ButtonVariants::_Secondary,
///         size: ButtonSize::_LG,
///         text: "Click me",
///         onclick: move |_| onclick(),
///     }
/// }
/// ```
///
/// ## Props
///
/// - `class`: String - an additional class for extra styling.
/// - `variant`: ButtonVariants - the visual style variant of the button.
/// - `size`: ButtonSize - the size of the button.
/// - `text`: String - the display text of the button.
/// - `onclick`: Callback - a callback function triggered on button click.
pub fn Button(props: ButtonProps) -> Element {
    // Destructure the props for easier access
    let ButtonProps {
        class,
        variant,
        size,
        text,
    } = props;

    // Match the provided variant to its corresponding CSS class
    let variant_class = match variant {
        ButtonVariants::Default => VARIANT_DEFAULT,
        ButtonVariants::_Secondary => VARIANT_SECONDARY,
        ButtonVariants::_Destructive => VARIANT_DESTRUCTIVE,
        ButtonVariants::_Outline => VARIANT_OUTLINE,
        ButtonVariants::_Ghost => VARIANT_GHOST,
        ButtonVariants::_Link => VARIANT_LINK,
        // Handle additional variants here
    };

    // Match the provided size to its corresponding CSS class
    let size_class = match size {
        ButtonSize::_SM => SIZE_SM,
        ButtonSize::_LG => SIZE_LG,
        ButtonSize::_ICON => SIZE_ICON,
        _ => SIZE_DEFAULT,
    };

    // Combine the base, variant, and size classes with any additional classes provided
    let combined_class = format!("{} {} {} {}", BASE_CLASS, variant_class, size_class, class);

    // Render the button element with the combined classes and text content
    rsx! {
        button { class: "{combined_class}", "{text}" }
    }
}

// Constants defining base, variant, and size classes for button styling
const BASE_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

const VARIANT_DEFAULT: &str = "bg-blue-200 text-primary-foreground shadow hover:bg-blue-800";
const VARIANT_SECONDARY: &str = "bg-green-200 text-secondary-foreground shadow-sm hover:bg-green-800";
const VARIANT_DESTRUCTIVE: &str = "bg-red-200 text-destructive-foreground shadow-sm hover:bg-red-800";
const VARIANT_OUTLINE: &str = "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground";
const VARIANT_GHOST: &str = "hover:bg-accent hover:text-accent-foreground";
const VARIANT_LINK: &str = "text-primary underline-offset-4 hover:underline";

const SIZE_DEFAULT: &str = "h-9 px-4 py-2";
const SIZE_SM: &str = "h-8 rounded-md px-3 text-xs";
const SIZE_LG: &str = "h-10 rounded-md px-8";
const SIZE_ICON: &str = "h-9 w-9";
