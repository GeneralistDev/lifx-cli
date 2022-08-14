use std::{io::{stdin, Write, stdout}, net::UdpSocket};
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
                            arg!(-d --duration [DURATION] "The time in seconds to make the state change over")
                                .default_value("0.0")
                        )
                        .arg(
                            arg!(-p --power [POWER] "Power state (on/off)")
                        )
                        .arg(
                            arg!(-c --color [COLOR] "Color of the light. See https://api.developer.lifx.com/v1/docs/colors for color documentation")
                        )
                        .arg(
                            arg!(-b --brightness [BRIGHTNESS] "Brightness between 0.0 and 1.0")
                        )
                        .arg(
                            arg!(-i --infrared [INFRARED] "The maximum brightness of the infrared channel from 0.0 to 1.0")
                        )
                        .arg(
                            arg!(-f --fast "Execute the query fast, without initial state checks and wait for no results.")
                        )
                )
                .arg(
                    arg!(-s --selector <SELECTOR> "Selector to filter lights. Omit to affect all lights. See https://api.developer.lifx.com/docs/selectors for selector documentation")
                        .default_value("all")
                )
        )
        .subcommand(
            Command::new("lan")
                .about("UDP LAN commands")
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

    if let Some(_) = matches.subcommand_matches("lan") {
        // TODO: Refactor UDP code out into reusable generic function
        debug!("LAN command test");

        let header = lifx::lan::Header::new(2, 2);

        let encoded_header: Vec<u8> = bincode::serialize(&header).unwrap();

        // let set_color_payload = lifx::lan::SetColorPayload {
        //     reserved1: 0,
        //     hue: 21845,
        //     saturation: 65535,
        //     brightness: 65535,
        //     kelvin: 3500,
        //     duration: 0,
        // };

        // let mut encoded_payload: Vec<u8> = bincode::serialize(&set_color_payload).unwrap();

        // encoded_header.append(&mut encoded_payload);

        for x in &encoded_header {
            print!("{:08b} ", x);
        }

        println!("");

        let socket = UdpSocket::bind("0.0.0.0:56701")?;

        socket.set_broadcast(true).expect("could not set socket to broadcast");

        socket.send_to(&encoded_header.as_slice(), "255.255.255.255:56700").expect("failed to send message");

        let mut buffer: [u8; 5] = [0; 5];

        let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("no data received");
        println!("{:?}", number_of_bytes);
        println!("{:?}", src_addr);

        // let decoded = bincode::deserialize::<lifx::lan::StateServiceResponse>(&buffer);

        // println!("{:?}", decoded);

        println!("Turning off light");

        let power_payload = lifx::lan::SetLightPowerPayload::new(true, 0);

        let header = lifx::lan::Header::new(3, 117);

        let mut encoded_header: Vec<u8> = bincode::serialize(&header).unwrap();
        
        let mut encoded_power_payload: Vec<u8> = bincode::serialize(&power_payload).unwrap();

        encoded_header.append(&mut encoded_power_payload);

        socket.send_to(&encoded_header, src_addr).expect("could not send command");
    }

    Ok(())
}
