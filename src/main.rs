use std::{io::{stdin, Write, stdout}, net::{SocketAddr, Ipv4Addr, IpAddr}};

use clap::{command, arg, Command, AppSettings};
use log::debug;
use system_config::Config;
use crate::lifx::lan::{LifxPacket::GetService, LifxPacket::SetPower, StateServiceResponse, SetLightPowerPayload};

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
                )
                .subcommand(
                    Command::new("toggle")
                        .about("Toggle a light")
                        .arg(
                            arg!(-d --duration "The time in seconds to spend perfoming the power toggle")
                                .default_value("0.0")
                        )
                )
                .subcommand(
                    Command::new("set-state")
                        .about("Set state of light/s")
                        .arg(
                            arg!(-d --duration [duration] "The time in seconds to make the state change over")
                                .default_value("0.0")
                        )
                        .arg(
                            arg!(-p --power [power] "Power state (on/off)")
                        )
                        .arg(
                            arg!(-c --color [color] "Color of the light. See https://api.developer.lifx.com/v1/docs/colors for color documentation")
                        )
                        .arg(
                            arg!(-b --brightness [brightness] "Brightness between 0.0 and 1.0")
                        )
                        .arg(
                            arg!(-i --infrared [infrared] "The maximum brightness of the infrared channel from 0.0 to 1.0")
                        )
                        .arg(
                            arg!(-f --fast "Execute the query fast, without initial state checks and wait for no results.")
                        )
                )
                .arg(
                    arg!(-s --selector <selector> "Selector to filter lights. Omit to affect all lights. See https://api.developer.lifx.com/docs/selectors for selector documentation")
                        .default_value("all")
                )
        )
        .subcommand(
            Command::new("lan")
                .about("UDP LAN commands")
                .subcommand(
                    Command::new("discover")
                        .about("Discover devices on network")
                )
                .subcommand(
                    Command::new("power")
                    .about("Manage light power")
                    .arg(
                        arg!(<state> "on/off")
                    )
                )
                .arg(
                    arg!(-i --ip [IP_Address] "The IP Address of the device to target for non-broadcast commands and queries")
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
        debug!("lights module");

        let lifx_commands: lifx::commands::LifxCommands = lifx::commands::LifxCommands::new(&api_key, &display_raw);

        let selector = matches.get_one::<String>("selector").unwrap();

        debug!("selector: {}", selector);

        if let Some(_) = matches.subcommand_matches("list") {
            debug!("list command");
            lifx_commands.list_lights(selector).await?;
        }

        if let Some(matches) = matches.subcommand_matches("toggle") {
            debug!("toggle command");
            let duration = matches.value_of_t::<f64>("duration").ok();
            lifx_commands.toggle_lights(selector, duration).await?;
        }

        if let Some(matches) = matches.subcommand_matches("set-state") {
            debug!("set-state command");
            let duration = matches.value_of_t::<f64>("duration").ok();
            let brightness = matches.value_of_t::<f64>("brightness").ok();
            let infrared = matches.value_of_t::<f64>("infrared").ok();
            let fast = matches.value_of_t::<bool>("fast").ok();
            let power = matches.get_one::<String>("power");
            let color = matches.get_one::<String>("color");

            lifx_commands.set_state(selector, power, color, brightness, duration, infrared, fast).await?;
        }
    }

    if let Some(matches) = matches.subcommand_matches("lan") {
        let lan_service = lifx::lan_service::LanService::new();

        let target_address = matches.get_one::<String>("ip");

        if let Some(_) = matches.subcommand_matches("discover") {
            let (data, ip_addr) = lan_service.broadcast_query(GetService, None).expect("Not to fail");

            let result = bincode::deserialize::<StateServiceResponse>(&data).unwrap();
            
            println!("Status: {:?}", result);
            println!("IP Address: {:?}", ip_addr);
        }

        if let Some(matches) = matches.subcommand_matches("power") {
            let power_state = matches.get_one::<String>("state").expect("Power state (on/off) is required");

            let power_payload = SetLightPowerPayload::new(power_state == "on", 0);

            let ipv4: Ipv4Addr = target_address.expect("Target address is required for this command")
                .parse()
                .expect("Unable to parse socket address");

            let addr = SocketAddr::new(IpAddr::V4(ipv4), 56700);

            lan_service.send_command(addr, SetPower, Box::new(power_payload));
        }
    }

    Ok(())
}
