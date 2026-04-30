use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct IMUEntry {
    pub ts: f64,
    pub id: String,
    #[serde(rename = "Ax")]
    pub ax: f64,
    #[serde(rename = "Ay")]
    pub ay: f64,
    #[serde(rename = "Az")]
    pub az: f64,
    #[serde(rename = "Gx")]
    pub gx: f64,
    #[serde(rename = "Gy")]
    pub gy: f64,
    #[serde(rename = "Gz")]
    pub gz: f64,
}

impl IMUEntry {
    fn calculate_acceleration(&self) -> u64 {
        ((self.ax.powi(2) + self.ay.powi(2) + self.az.powi(2)).sqrt() * 100.0) as u64
    }
}

pub fn load_csv(path: std::path::PathBuf) -> Result<Vec<IMUEntry>, Box<dyn Error>> {
    let mut imu_reader = csv::Reader::from_path(path)?;

    // collect requires explicit type hint because it is generic
    let imu_records: Vec<IMUEntry> = imu_reader
        .deserialize()
        .collect::<Result<Vec<IMUEntry>, csv::Error>>()?;
    Ok(imu_records)
}

pub fn calculate_accelerations(readings: &[IMUEntry]) -> Vec<u64> {
    readings
        .iter()
        .map(|imu_reading| imu_reading.calculate_acceleration())
        .collect()
}
