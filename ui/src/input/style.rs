//ui/input/style.rs

use super::props::{InputSize, InputVariants};

pub const BASE_CLASS: &str = "flex h-9 rounded-md border border-input bg-transparent text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50";

const VARIANT_DEFAULT: &str = "bg-white";
const VARIANT_OUTLINE: &str = "border-2";
const VARIANT_UNDERLINE: &str = "border-b-2 border-t-0 border-l-0 border-r-0";
const VARIANT_FLUSHED: &str = "border-b-2";
const VARIANT_FILLED: &str = "bg-gray-200";

const SIZE_DEFAULT: &str = "px-3 py-1";
const SIZE_SM: &str = "text-xs px-2 py-0.5";
const SIZE_LG: &str = "text-lg px-4 py-2";

pub fn get_variant_class(variant: &InputVariants) -> &str {
    match variant {
        InputVariants::Default => VARIANT_DEFAULT,
        InputVariants::_Outline => VARIANT_OUTLINE,
        InputVariants::_Filled => VARIANT_FILLED,
        InputVariants::_Flushed => VARIANT_FLUSHED,
        InputVariants::_Underline => VARIANT_UNDERLINE,
    }
}

// Function to get the size CSS class
pub fn get_size_class(size: &InputSize) -> &str {
    match size {
        InputSize::_SM => SIZE_SM,
        InputSize::_LG => SIZE_LG,
        _ => SIZE_DEFAULT,
    }
}
