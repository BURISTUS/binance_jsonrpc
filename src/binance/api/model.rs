use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct DepthStreamData {
    #[serde(deserialize_with = "de_float_from_str")]
    pub c: f32,
    pub s: String,
}

pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}

#[derive(Debug, Deserialize)]
pub struct DepthStreamWrapper {
    pub stream: String,
    pub data: DepthStreamData,
}
