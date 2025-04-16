use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

pub struct Feature {
    pub name: String,
    pub feature_type: FeatureType,
    pub properties: HashMap<String, String>,
    pub initial_state: HashMap<String, String>,
}

#[derive(Clone, Copy)]
pub enum FeatureType {
    ApplicationModifier,
    CoreFeature,
    Entity,
    UiFeature,
}

impl FeatureType {
    pub fn as_str(&self) -> &str {
        match self {
            FeatureType::ApplicationModifier => "application-modifier",
            FeatureType::CoreFeature => "core-feature",
            FeatureType::Entity => "entity",
            FeatureType::UiFeature => "ui-feature",
        }
    }

    pub fn get_template_dest_dir(&self) -> &str {
        match self {
            FeatureType::ApplicationModifier => "src/app/core/application-modifiers",
            FeatureType::CoreFeature => "src/app/core/features",
            FeatureType::Entity => "src/app/core/entity",
            FeatureType::UiFeature => "src/app/features",
        }
    }
}

impl Display for FeatureType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.as_str())
    }
}
