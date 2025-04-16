mod commands;
mod core;

use clap::Command;

fn main() {
    create_command_router();
}

fn create_command_router() {
    let command = Command::new("Sniffies CLI");

    let command = command.subcommand(
        Command::new("create")
            .short_flag_alias('c')
            .display_name("Create module")
            .about("Create a new module (app modifier, core feature, entity or  UI feature)"),
    );

    match command.get_matches().subcommand_name() {
        Some("create") => commands::create(),
        _ => {
            println!("No command specified");
        }
    }
}
