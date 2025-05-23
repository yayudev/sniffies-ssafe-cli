use std::collections::HashMap;

use convert_case::{Case, Casing};

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

    // TODO: Implement FactoryForEntity
    ModelForEntity,
    ServiceForEntity,
    ServiceTestForEntity,
    StoreForEntity,
    StoreTestForEntity,
    IndexForEntity,
}

impl Template {
    pub fn get_content(&self) -> &str {
        match self {
            /************
             * Features *
             ************/
            Template::Facade => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/facade.ts"))
            }
            Template::FacadeTest => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/templates/facade.spec.ts"
            )),
            Template::Model => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/model.ts"))
            }
            Template::Service => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/service.ts"))
            }
            Template::ServiceTest => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/templates/service.spec.ts"
            )),
            Template::Store => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/store.ts"))
            }
            Template::StoreTest => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/templates/store.spec.ts"
            )),
            Template::Index => {
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/index.ts"))
            }

            /************
             * Entities *
             ************/
            Template::ModelForEntity => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/templates/entity/model.ts"
            )),
            Template::ServiceForEntity => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/templates/entity/service.ts"
            )),
            Template::ServiceTestForEntity => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/templates/entity/service.spec.ts"
            )),
            Template::StoreForEntity => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/templates/entity/store.ts"
            )),
            Template::StoreTestForEntity => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/templates/entity/store.spec.ts"
            )),
            Template::IndexForEntity => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/templates/entity/index.ts"
            )),
        }
    }

    pub fn get_output_filename(&self, feature_name: &String) -> String {
        let feature_name = feature_name.to_case(Case::Kebab);

        match self {
            Template::Facade => format!("{}.facade.ts", feature_name),
            Template::FacadeTest => format!("{}.facade.spec.ts", feature_name),
            Template::Model => format!("{}.model.ts", feature_name),
            Template::Service => format!("{}.service.ts", feature_name),
            Template::ServiceTest => format!("{}.service.spec.ts", feature_name),
            Template::Store => format!("{}.store.ts", feature_name),
            Template::StoreTest => format!("{}.store.spec.ts", feature_name),
            Template::Index => "index.ts".to_string(),

            // Template::FactoryForEntity => format!("{}.factory.ts", feature_name),
            Template::ModelForEntity => format!("{}.model.ts", feature_name),
            Template::ServiceForEntity => format!("{}.service.ts", feature_name),
            Template::ServiceTestForEntity => format!("{}.service.spec.ts", feature_name),
            Template::StoreForEntity => format!("{}.store.ts", feature_name),
            Template::StoreTestForEntity => format!("{}.store.spec.ts", feature_name),
            Template::IndexForEntity => "index.ts".to_string(),
        }
    }
}

pub struct TemplateConfig {
    pub feature_name: String,
    pub feature_type: FeatureType,
    pub root_path: String,
}

pub fn create_config_for_template(config: &TemplateConfig, template: Template) -> RenderConfig {
    // TODO: Fetch this from config file
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
        pluralize: matches!(&feature.feature_type, FeatureType::Entity),
        feature,
        template,
        root_path: config.root_path.clone(),
    }
}
