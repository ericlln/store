use serde::Serialize;

#[derive(Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize)]
pub struct Space {
    pub id: i64,
    pub name: String,
    pub drawing: Vec<Vec<(f32, f32)>>
}

#[derive(Serialize)]
pub struct Store {
    pub id: i64,
    pub name: String,
    pub location: Point
}

#[derive(Serialize)]
pub struct Item {
    pub id: i64,
    #[serde(rename = "storeId")]
    pub store_id: i64,
    pub name: String,
    pub quantity: i32,
}