use super::props::TemplateVariants;

pub const BASE_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap  text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";

const VARIANT_DEFAULT: &str = "bg-blue-200 text-primary-foreground  hover:bg-blue-800";
const VARIANT_SECONDARY: &str = "bg-green-200 text-secondary-foreground  hover:bg-green-800";

pub fn get_variant_class(variant: &TemplateVariants) -> &str {
    match variant {
        TemplateVariants::Default => VARIANT_DEFAULT,
        TemplateVariants::_Secondary => VARIANT_SECONDARY,
    }
}
