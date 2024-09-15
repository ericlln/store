use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize)]
pub struct Store {
    pub name: String,
    pub path: String,
    pub available: bool
}

#[derive(Serialize)]
pub struct Space {
    pub id: i64,
    pub name: String,
    pub drawing: Option<Vec<Vec<Point>>>
}

#[derive(Serialize)]
pub struct Bin {
    pub id: i64,
    #[serde(rename="spaceId")]
    pub space_id: i64,
    pub name: String,
    pub location: Point
}

#[derive(Serialize)]
pub struct Item {
    pub id: i64,
    pub name: String,
    #[serde(rename = "spaceId")]
    pub space_id: i64,
    #[serde(rename = "spaceName")]
    pub space_name: Option<String>,
    #[serde(rename = "binId")]
    pub bin_id: i64,
    #[serde(rename = "binName")]
    pub bin_name: Option<String>,
    pub quantity: Option<i32>,
    pub notes: Option<String>
}