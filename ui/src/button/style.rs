//ui/button/style.rs

use super::{ButtonSize, ButtonVariants};

pub const BASE_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

const VARIANT_DEFAULT: &str = "bg-blue-200 text-primary-foreground shadow hover:bg-blue-800";
const VARIANT_SECONDARY: &str =
    "bg-green-200 text-secondary-foreground shadow-sm hover:bg-green-800";
const VARIANT_DESTRUCTIVE: &str =
    "bg-red-200 text-destructive-foreground shadow-sm hover:bg-red-800";
const VARIANT_OUTLINE: &str =
    "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground";
const VARIANT_GHOST: &str = "hover:bg-accent hover:text-accent-foreground";
const VARIANT_LINK: &str = "text-primary underline-offset-4 hover:underline";

const SIZE_DEFAULT: &str = "h-9 px-4 py-2";
const SIZE_SM: &str = "h-8 rounded-md px-3 text-xs";
const SIZE_LG: &str = "h-10 rounded-md px-8";
const SIZE_ICON: &str = "h-9 w-9";

pub fn get_variant_class(variant: &ButtonVariants) -> &str {
    match variant {
        ButtonVariants::Default => VARIANT_DEFAULT,
        ButtonVariants::_Secondary => VARIANT_SECONDARY,
        ButtonVariants::_Destructive => VARIANT_DESTRUCTIVE,
        ButtonVariants::_Outline => VARIANT_OUTLINE,
        ButtonVariants::_Ghost => VARIANT_GHOST,
        ButtonVariants::_Link => VARIANT_LINK,
    }
}

// Function to get the size CSS class
pub fn get_size_class(size: &ButtonSize) -> &str {
    match size {
        ButtonSize::_SM => SIZE_SM,
        ButtonSize::_LG => SIZE_LG,
        ButtonSize::_ICON => SIZE_ICON,
        _ => SIZE_DEFAULT,
    }
}
