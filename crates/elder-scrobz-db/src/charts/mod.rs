use serde::Deserialize;
use utoipa::ToSchema;

pub mod album;
pub mod artists;
pub mod tracks;

#[derive(Debug, ToSchema, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Period {
    Week,
    Month,
    Year,
    Today,
}
