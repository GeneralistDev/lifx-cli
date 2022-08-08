use std::collections::HashMap;

use super::types::{ListLightResponse, ToggledLightsResponse};

use lifx_cli::SerializeToTable;
use log::debug;
use prettytable::{Table, format};
use reqwest::Client;
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
}
