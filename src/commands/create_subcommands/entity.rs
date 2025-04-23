use crate::core::templating::render_template::render_template;
use crate::core::templating::template::{Template, TemplateConfig, create_config_for_template};

pub fn entity(config: &TemplateConfig) {
    let files = vec![
        create_config_for_template(config, Template::ModelForEntity),
        create_config_for_template(config, Template::ServiceForEntity),
        create_config_for_template(config, Template::ServiceTestForEntity),
        create_config_for_template(config, Template::StoreForEntity),
        create_config_for_template(config, Template::StoreTestForEntity),
        create_config_for_template(config, Template::IndexForEntity),
    ];

    for file_config in files {
        render_template(&file_config).expect("Failed to write file");
    }
}
