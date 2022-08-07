use std::io::{stdin, Write, stdout};
use log::debug;
use system_config::Config;

#[macro_use] extern crate prettytable;
mod lifx;

const API_KEY_CONFIG_KEY: &str = "api_key";

// struct Arguments {
//     object: String,
//     command: String,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut config  = Config::new("lifx-cli-config").unwrap();

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

    let lifx_commands: lifx::commands::LifxCommands = lifx::commands::LifxCommands::new(&api_key);

    lifx_commands.list_lights().await?;

    Ok(())
}
