use lifx_cli::SerializeToTable;
use prettytable::{Table, format};
use ansi_rgb::Background;
use rgb::RGB8;
use hsl::HSL;

use super::types::{ListLightResponse, ToggledLightsResponse, SetStateResponse};

impl SerializeToTable for ListLightResponse {
    fn serialize_row(&self, table: &mut Table) {
        let hsl = HSL {
            h: self.color.hue,
            s: self.color.saturation * 100.0,
            l: if self.color.saturation == 0.0 { 1.1 } else { 0.5 },
        };

        let (r, g, b) = hsl.to_rgb();

        table.add_row(row![
            format!("{}", self.id),
            format!("{}", self.label),
            format!("{}", self.connected),
            format!("{}", self.power),
            format!("{}%", self.brightness * 100.0),
            "     ".bg(RGB8::new(r, g, b)),
        ]);
    }
}

impl SerializeToTable for ToggledLightsResponse {
    fn serialize_row(&self, table: &mut Table) {
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        table.add_row(row![
            b -> "ID",
            b -> "Status",
            b -> "Label",
            b -> "Power",
        ]);

        for result in &self.results {
            table.add_row(row![
                result.id.as_ref().unwrap_or(&"".to_owned()),
                result.status.as_ref().unwrap_or(&"".to_owned()),
                result.label.as_ref().unwrap_or(&"".to_owned()),
                result.power.as_ref().unwrap_or(&"".to_owned()),
            ]);
        }
    }
}

impl SerializeToTable for SetStateResponse {
    fn serialize_row(&self, table: &mut Table) {
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        table.add_row(row![
            b -> "ID",
            b -> "Status",
            b -> "Label",
        ]);

        for result in &self.results {
            table.add_row(row![
                result.id.as_ref().unwrap_or(&"".to_owned()),
                result.status.as_ref().unwrap_or(&"".to_owned()),
                result.label.as_ref().unwrap_or(&"".to_owned()),
            ]);
        }
    }
}