use std::collections::HashMap;

use clap::builder::Str;

use super::{
    feature::{Feature, FeatureType},
    render_config::RenderConfig,
};

pub enum Template {
    Facade,
    FacadeTest,
    Model,
    Service,
    ServiceTest,
    Store,
    StoreTest,
    Index,
}

impl Template {
    pub fn get_template_filename(&self) -> &str {
        match self {
            Self::Facade => "facade.ts",
            Self::FacadeTest => "facade.spec.ts",

            Self::Model => "model.ts",
            Self::Service => "service.ts",
            Self::ServiceTest => "service.spec.ts",

            Self::Store => "store.ts",
            Self::StoreTest => "store.spec.ts",

            Self::Index => "index.ts",
        }
    }
}

pub struct TemplateConfig {
    pub feature_name: String,
    pub feature_type: FeatureType,
    pub state_file_path: String,
    pub root_path: String,
}

pub fn create_config_for_template(config: &TemplateConfig, template: Template) -> RenderConfig {
    let mut properties: HashMap<String, String> = HashMap::new();
    properties.insert("radius".to_string(), "number".to_string());
    properties.insert("canRender".to_string(), "boolean".to_string());

    let mut initial_state: HashMap<String, String> = HashMap::new();
    initial_state.insert("radius".to_string(), "42.069".to_string());
    initial_state.insert("canRender".to_string(), "true".to_string());

    let feature = Feature {
        name: config.feature_name.clone(),
        feature_type: config.feature_type,
        properties,
        initial_state,
    };

    RenderConfig {
        feature,
        template,
        root_path: config.root_path.clone(),
    }
}
