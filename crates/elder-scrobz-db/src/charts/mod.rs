use serde::Deserialize;

pub mod album;
pub mod artists;
pub mod tracks;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Period {
    Week,
    Month,
    Year,
    Today,
}
