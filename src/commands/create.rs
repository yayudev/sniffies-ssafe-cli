use inquire::{Select, Text};

use crate::core::templating::feature::FeatureType;
use crate::core::templating::template::TemplateConfig;

use super::create_subcommands;

pub fn create() {
    let config = prompt_feature_config().expect("Failed to read config");

    match config.feature_type {
        FeatureType::ApplicationModifier => create_subcommands::application_modifier(&config),
        FeatureType::CoreFeature => create_subcommands::core_feature(&config),
        FeatureType::Entity => create_subcommands::entity(&config),
        FeatureType::UiFeature => create_subcommands::ui_feature(&config),
    }
}

fn prompt_feature_config() -> Result<TemplateConfig, Box<dyn std::error::Error>> {
    let feature_options = vec![
        FeatureType::ApplicationModifier,
        FeatureType::CoreFeature,
        FeatureType::Entity,
        FeatureType::UiFeature,
    ];

    let feature_name = Text::new("Feature name: ").prompt()?;
    let feature_type = Select::new("Select feature type: ", feature_options).prompt()?;
    // let state_file_path = Text::new("Select state file: ")
    //     .with_placeholder("./test.ts")
    //     .with_default("./test.ts")
    //     .prompt()?;
    let root_path = Text::new("Project root path")
        .with_default("./dist")
        .with_placeholder("./dist")
        .prompt()?;

    Ok(TemplateConfig {
        feature_name,
        feature_type,
        //state_file_path,
        root_path,
    })
}
