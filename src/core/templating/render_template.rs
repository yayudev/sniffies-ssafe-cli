use std::{fs, path::Path};

use convert_case::{Case, Casing};
use tera::{Context, Tera};

use super::{filters, render_config::RenderConfig, template::Template};

pub fn render_template(config: &RenderConfig) -> Result<(), std::io::Error> {
    let template_content = read_template_file(&config.template);

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![("template", &template_content)])
        .expect("failed to add raw template");
    let mut context = Context::new();

    // Register variables
    context.insert("feature_name", &config.feature.name);
    context.insert("properties", &config.feature.properties);
    context.insert("initial_state", &config.feature.initial_state);

    // Register custoom filters
    tera.register_filter("pascal", filters::pascal_filter);
    tera.register_filter("kebab", filters::kebab_filter);
    tera.register_filter("constant", filters::constant_filter);

    // Render template
    let generated_code = tera
        .render("template", &context)
        .expect("Failed to render template");

    write_to_feature_file(generated_code, config)
}

fn write_to_feature_file(
    generated_code: String,
    config: &RenderConfig,
) -> Result<(), std::io::Error> {
    let output_dir = Path::join(
        Path::new(&config.root_path),
        config.feature.feature_type.clone().get_template_dest_dir(),
    );
    let output_dir = Path::join(
        output_dir.as_path(),
        config.feature.name.to_case(Case::Kebab),
    );

    if !fs::exists(&output_dir).unwrap_or(false) {
        fs::create_dir_all(&output_dir).expect("Failed to create output dir");
    }

    let filename = match config.template {
        // Index doesn't need prefix
        Template::Index => config.template.get_template_filename().to_string(),

        // Format to {feature_name}.{file-type.ts}
        _ => format!(
            "{}.{}",
            &config.feature.name.to_case(Case::Kebab),
            config.template.get_template_filename(),
        ),
    };

    let output_path = Path::join(Path::new(&output_dir), Path::new(&filename));
    println!(
        "File written: {}",
        output_path
            .clone()
            .into_os_string()
            .into_string()
            .expect("Failed_to_parse_string")
    );

    fs::write(output_path, generated_code)
}

fn read_template_file(template: &Template) -> String {
    let facade_template = template.get_template_filename();
    let template_path = format!("templates/{}", facade_template);

    fs::read_to_string(&template_path)
        .unwrap_or_else(|_| panic!("Error reading file: {}", template_path))
}
