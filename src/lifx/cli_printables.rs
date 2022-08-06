use lifx_cli::SerializeToTable;
use prettytable::Table;
use ansi_rgb::Background;
use rgb::RGB8;
use hsl::HSL;

use super::types::ListLightResponse;

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