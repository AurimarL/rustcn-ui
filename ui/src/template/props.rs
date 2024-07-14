use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct TemplateProps {
    #[props(default)]
    pub variant: TemplateVariants,
    // Additional props can be added here.
}

#[derive(PartialEq, Clone)]
pub enum TemplateVariants {
    Default,
    _Secondary,
    // Additional variants can be added here.
}

impl Default for TemplateVariants {
    fn default() -> Self {
        TemplateVariants::Default
    }
}
