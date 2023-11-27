use chrono::{DateTime, Utc};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DataFile {
    pub job_name: Option<String>,
    pub data_points: Vec<DataPoint>,
}

#[derive(Deserialize, Debug)]
pub struct DataPoint {
    pub count: u32,
    pub timestamp: DateTime<Utc>,
}
