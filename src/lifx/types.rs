use serde_derive::Deserialize;
use serde_derive::Serialize;
use optional_field::{Field, serde_optional_fields};

// region: ListLightResponse

#[serde_optional_fields]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLightResponse {
    pub id: String,
    pub uuid: String,
    pub label: String,
    pub connected: bool,
    pub power: String,
    pub color: Color,
    pub brightness: f64,
    pub effect: Field<String>,
    pub group: Group,
    pub location: Location,
    pub product: Product,
    #[serde(rename = "last_seen")]
    pub last_seen: String,
    #[serde(rename = "seconds_since_seen")]
    pub seconds_since_seen: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub hue: f64,
    pub saturation: f64,
    pub kelvin: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub name: String,
    pub identifier: String,
    pub company: String,
    #[serde(rename = "vendor_id")]
    pub vendor_id: i64,
    #[serde(rename = "product_id")]
    pub product_id: i64,
    pub capabilities: Capabilities,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    #[serde(rename = "has_color")]
    pub has_color: bool,
    #[serde(rename = "has_variable_color_temp")]
    pub has_variable_color_temp: bool,
    #[serde(rename = "has_ir")]
    pub has_ir: bool,
    #[serde(rename = "has_hev")]
    pub has_hev: bool,
    #[serde(rename = "has_chain")]
    pub has_chain: bool,
    #[serde(rename = "has_matrix")]
    pub has_matrix: bool,
    #[serde(rename = "has_multizone")]
    pub has_multizone: bool,
    #[serde(rename = "min_kelvin")]
    pub min_kelvin: f64,
    #[serde(rename = "max_kelvin")]
    pub max_kelvin: f64,
}

// endregion: ListLightResponse

// region: ToggledLightsResponse

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggledLightsResponse {
    pub results: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub id: Option<String>,
    pub label: Option<String>,
    pub status: Option<String>,
    pub power: Option<String>,
}

// endregion: ToggledLightsResponse

// region: SetStateResponse

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetStateResponse {
    pub results: Vec<Result>,
}

// endregion: SetStateResponse

// region: InvalidColorResponse

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvalidColorResponse {
    pub error: String,
    pub errors: Vec<Error>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub field: String,
    pub message: Vec<String>,
}

// endregion: InvalidColorResponse