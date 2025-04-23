use super::{feature::Feature, template::Template};

pub struct RenderConfig {
    pub feature: Feature,
    pub template: Template,
    pub root_path: String,
    pub pluralize: bool,
}
