use std::collections::HashMap;

use super::types::{ListLightResponse, ToggledLightsResponse, InvalidColorResponse, SetStateResponse};

use lifx_cli::SerializeToTable;
use log::debug;
use prettytable::{Table, format};
use reqwest::{Client, StatusCode};
use serde_json::{Map, Value, Number};
use urlencoding::encode;

const LIFX_URL_TEMPLATE: &str = "https://api.lifx.com/v1";

pub struct LifxCommands {
    token: String,
    display_raw: bool,
}

impl LifxCommands {
    pub fn new(key: &str, raw: &bool) -> LifxCommands {
        LifxCommands { token: String::from(key), display_raw: *raw }
    }

    /*
        https://api.developer.lifx.com/docs/list-lights
    */
    pub async fn list_lights(&self, selector: &String) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();

        debug!("Sending request");

        let res: reqwest::Response = client
            .get(format!("{}{}", LIFX_URL_TEMPLATE, "/lights/".to_owned() + encode(selector).into_owned().as_str()))
            .bearer_auth(&self.token)
            .send()
            .await?;

        debug!("Parsing json");

        let lights = res.json::<Vec<ListLightResponse>>().await?;

        debug!("{}", serde_json::to_string_pretty(&lights)?);

        if self.display_raw {
            println!("{}", serde_json::to_string_pretty(&lights)?);
        } else {
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    
            table.add_row(row![
                b -> "ID",
                b -> "Label",
                b -> "Connected",
                b -> "Power",
                b -> "Brightness",
                b -> "Color",
            ]);
    
            for light in lights {
                light.serialize_row(&mut table);
            }
    
            table.printstd();
        }

        Ok(())
    }

    pub async fn toggle_lights(&self, selector: &String, duration: Option<f64>) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();

        debug!("Sending request");

        let mut body = HashMap::new();

        if let Some(duration) = duration {
            debug!("duration: {}", duration.to_string());
            body.insert("duration".to_string(), duration.to_string());
        }

        debug!("{:?}", body);

        let res: reqwest::Response = client
            .post(format!("{}{}", LIFX_URL_TEMPLATE, "/lights/".to_owned() + encode(selector).into_owned().as_str() + "/toggle"))
            .json(&body)
            .bearer_auth(&self.token)
            .send()
            .await?;

        debug!("Light/s toggled");

        let toggle_results = res.json::<ToggledLightsResponse>().await?;

        if self.display_raw {
            println!("{}", serde_json::to_string_pretty(&toggle_results)?);
        } else {
            let mut table = Table::new();
            toggle_results.serialize_row(&mut table);
            table.printstd();
        }

        Ok(())
    }

    pub async fn set_state(
        &self,
        selector: &String,
        power: Option<&String>,
        color: Option<&String>,
        brightness: Option<f64>,
        duration: Option<f64>,
        infrared: Option<f64>,
        fast: Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();

        let mut body = Map::new();

        // Validate the color if that's provided
        if let Some(color) = color {
            let client = Client::new();

            let res: reqwest::Response = client
                .get(format!("{}{}", LIFX_URL_TEMPLATE, "/color"))
                .query(&[("string", color)])
                .bearer_auth(&self.token)
                .send()
                .await?;

            if res.status() == StatusCode::UNPROCESSABLE_ENTITY {
                let color_validation_errors = res.json::<InvalidColorResponse>().await?;
                println!("{}", color_validation_errors.error);
                for error in color_validation_errors.errors {
                    println!("{:?}", error);
                    return Ok(())
                }
            }

            body.insert("color".to_string(), Value::String(color.to_string()));
        };

        if let Some(power) = power {
            match &power[..] {
                "on" | "off" => {
                    body.insert("power".to_string(), Value::String(power.to_string()));
                },
                _ => {
                    println!("'power' should either be 'on' or 'off'");
                    return Ok(());
                }
            }
        }

        if let Some(brightness) = brightness {
            if brightness < 0.0 && brightness > 1.0 {
                println!("'brightness' must be between 0.0 and 1.0");
                return Ok(());
            }

            body.insert("brightness".to_string(), Value::Number(Number::from_f64(brightness).unwrap()));
        }

        if let Some(duration) = duration {
            if duration < 0.0 && duration > 3155760000.0 {
                println!("'duration' must be between 0.0 and 3155760000.0");
                return Ok(());
            }

            body.insert("duration".to_string(), Value::Number(Number::from_f64(duration).unwrap()));
        }

        if let Some(infrared) = infrared {
            if infrared < 0.0 && infrared > 1.0 {
                println!("'infrared' must be between 0.0 and 1.0");
                return Ok(());
            }

            body.insert("infrared".to_string(), Value::Number(Number::from_f64(infrared).unwrap()));
        }

        if let Some(fast) = fast {
            body.insert("fast".to_string(), Value::Bool(fast));
        }

        let res: reqwest::Response = client
            .put(format!("{}{}", LIFX_URL_TEMPLATE, "/lights/".to_owned() + encode(selector).into_owned().as_str() + "/state"))
            .json(&body)
            .bearer_auth(&self.token)
            .send()
            .await?;

        let set_state_response = res.json::<SetStateResponse>().await?;

        if self.display_raw {
            println!("{}", serde_json::to_string_pretty(&set_state_response)?);
        } else {
            let mut table = Table::new();
            set_state_response.serialize_row(&mut table);
            table.printstd();
        }

        Ok(())
    }
}
