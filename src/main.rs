use std::{io::{stdin, Write, stdout}};
use clap::{command, arg, Command, AppSettings};
use log::debug;
use system_config::Config;

#[macro_use] extern crate prettytable;
mod lifx;

const API_KEY_CONFIG_KEY: &str = "api_key";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut config  = Config::new("lifx-cli-config").unwrap();

    let command = command!()
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            Command::new("auth")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .about("Authentication module")
                .subcommand(
                    Command::new("clear")
                    .about("clear the API key")
                )
        )
        .subcommand(
            Command::new("lights")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .about("Lights module")
                .subcommand(
                    Command::new("list")
                        .about("List lights")
                        .arg(
                            arg!(-s --selector "Selector to filter lights. Omit to see all lights")
                                .takes_value(true)
                                .help("See https://api.developer.lifx.com/docs/selectors for selector documentation")
                        )
                )
        )
        .arg(
            arg!(-r --raw "Display raw json response data instead of tables")
                .takes_value(false)
        );

    let matches = &command.get_matches();

    if let Some(matches) = matches.subcommand_matches("auth") {
        if let Some(_) = matches.subcommand_matches("clear") {
            config.clear();
            println!("API key cleared");
        } else {
            return Ok(());
        }
    }

    debug!("Getting API Key");

    let api_key = match config.get(API_KEY_CONFIG_KEY) {
        Some(key_input) => {
            debug!("Found key {}", key_input);
            key_input
        },
        None => {
            print!("Enter your LIFX API Key: ");
            stdout().flush().unwrap();

            let mut input = String::new();

            stdin().read_line(&mut input)?;

            let trimmed = input.trim_end();

            config.insert(API_KEY_CONFIG_KEY, &trimmed.clone());
            config.write()?;
            String::from(trimmed)
        },
    };

    let display_raw: bool = matches.contains_id("raw");

    if let Some(matches) = matches.subcommand_matches("lights") {
        let lifx_commands: lifx::commands::LifxCommands = lifx::commands::LifxCommands::new(&api_key, &display_raw);

        if let Some(matches) = matches.subcommand_matches("list") {
            let default_selector = String::from("all");
            let selector = matches.get_one::<String>("selector").unwrap_or(&default_selector);

            lifx_commands.list_lights(selector).await?;
        }
    }

    Ok(())
}
