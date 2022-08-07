use super::types::ListLightResponse;

use lifx_cli::SerializeToTable;
use prettytable::{Table, format};
use reqwest::Client;

const LIFX_URL_TEMPLATE: &str = "https://api.lifx.com/v1";

pub struct LifxCommands {
    token: String,
}

impl LifxCommands {
    pub fn new(key: &str) -> LifxCommands {
        LifxCommands { token: String::from(key) }
    }

    /*
        https://api.developer.lifx.com/docs/list-lights
    */
    pub async fn list_lights(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        
        let client = Client::new();

        log::debug!("Sending request");

        let res: reqwest::Response = client
            .get(format!("{}{}", LIFX_URL_TEMPLATE, "/lights/all"))
            .bearer_auth(&self.token)
            .send()
            .await?;

        log::debug!("Parsing json");

        let lights = res.json::<Vec<ListLightResponse>>().await?;

        log::debug!("{}", serde_json::to_string_pretty(&lights)?);

        println!("All your lights...");

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

        Ok(())
    }
}


