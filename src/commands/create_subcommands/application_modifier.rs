use crate::core::templating::render_template::render_template;
use crate::core::templating::template::{Template, TemplateConfig, create_config_for_template};

pub fn application_modifier(config: &TemplateConfig) {
    let files = vec![
        create_config_for_template(config, Template::Facade),
        create_config_for_template(config, Template::FacadeTest),
        create_config_for_template(config, Template::Model),
        create_config_for_template(config, Template::Service),
        create_config_for_template(config, Template::ServiceTest),
        create_config_for_template(config, Template::Store),
        create_config_for_template(config, Template::StoreTest),
        create_config_for_template(config, Template::Index),
    ];

    for file_config in files {
        render_template(&file_config).expect("Failed to write file");
    }
}
