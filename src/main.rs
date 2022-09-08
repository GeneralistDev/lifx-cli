use std::{io::{stdin, Write, stdout}, net::{SocketAddr, Ipv4Addr, IpAddr}, process};

use clap::{command, arg, Command, AppSettings};
use log::debug;
use system_config::Config;
use crate::lifx::lan::{LifxPacket::GetService, LifxPacket::SetPower, LifxPacket::SetColor, StateServiceResponse, SetLightPowerPayload, SetLightColorPayload};

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
                .subcommand(
                    Command::new("color")
                    .about("Set light color")
                    .arg(
                        arg!(-h --hue <hue> "The section of the color spectrum that represents the color of your device. So for example red is 0, green is 120, etc")
                    )
                    .arg(
                        arg!(-s --saturation <saturation> "How strong the color is. So a zero saturation is completely white, whilst full saturation is the full color")
                    )
                    .arg(
                        arg!(-b --brightness <brightness> "How bright the color is. So zero brightness is the same as the device is off, while full brightness be just that.")
                    )
                    .arg(
                        arg!(-k --kelvin <kelvin> "The \"temperature\" when the device has zero saturation. So a higher value is a cooler white (more blue) whereas a lower value is a warmer white (more yellow)")
                    )
                    .arg(
                        arg!(-d --duration <duration> "Duration over which the color should change in milliseconds")
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

        if let Some(matches) = matches.subcommand_matches("color") {
            let hue = matches.value_of_t::<u16>("hue").expect("Hue is required");
            let saturation = matches.value_of_t::<u16>("saturation").expect("Saturation is required");
            let brightness = matches.value_of_t::<u16>("brightness").expect("Brightness is required");
            let kelvin = matches.value_of_t::<u16>("kelvin");
            let duration = matches.value_of_t::<u32>("duration").unwrap_or(0);

            // Some validation
            let mut validation_err = false;

            if hue > 360 {
                print!("Hue should be between 0 and 360");
                validation_err = true;
            }
            if saturation > 1 {
                print!("Saturation should be between 0 and 1");
                validation_err = true;
            }
            if brightness > 1 {
                print!("Brightness should be between 0 and 1");
                validation_err = true;
            }

            if validation_err {
                process::exit(1)
            }

            let hue_param = (((0x10000 as f64 * hue as f64) / 360.0) % 0x10000 as f64).round() as u16;
            let saturation_param = (0xFFFF as f64 * saturation as f64).round() as u16;
            let brightness_param = (0xFFFF as f64 * brightness as f64).round() as u16;
            let kelvin_param = match kelvin {
                Ok(kelvin) => if saturation_param == 0 { kelvin } else { 0 },
                _ => 0,
            };

            let payload = SetLightColorPayload::new(hue_param, saturation_param, brightness_param, kelvin_param, duration);

            let ipv4: Ipv4Addr = target_address.expect("Target address is required for this command")
                .parse()
                .expect("Unable to parse socket address");

            let addr = SocketAddr::new(IpAddr::V4(ipv4), 56700);

            lan_service.send_command(addr, SetColor, Box::new(payload));
        }
    }

    Ok(())
}
