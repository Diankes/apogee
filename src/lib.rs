use clap::Parser;
use serde::Deserialize;
use std::error::Error;

/// Opting for Clap derive option since it is more idiomatic and allows to type the file path
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Flight controller log parser",
    long_about = "Apogee: Flight Controller log parser"
)]
struct Args {
    /// Path to the input CSV file
    ///
    /// The headers of the file must be in the following format:
    ///
    ///   ts      - timestamp (seconds)
    ///   id      - sensor identifier (e.g. IMU0)
    ///   Ax,Ay,Az - accelerometer X/Y/Z (m/s²)
    ///   Gx,Gy,Gz - gyroscope X/Y/Z (rad/s)
    ///
    /// Example:
    ///   -0.755, IMU0, -9.62, -1.59, -0.91, -0.42, -0.21, 1.12
    #[arg(short, long, verbatim_doc_comment)]
    file: std::path::PathBuf,
}
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
    fn load_csv(path: std::path::PathBuf) -> Result<Vec<IMUEntry>, Box<dyn Error>> {
        let mut imu_reader = csv::Reader::from_path(path)?;

        // collect requires explicit type hint because it is generic
        let imu_records: Vec<IMUEntry> = imu_reader
            .deserialize()
            .collect::<Result<Vec<IMUEntry>, csv::Error>>()?;
        Ok(imu_records)
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let imu_readings = IMUEntry::load_csv(args.file)?;
    for imu_reading in imu_readings.iter().take(5) {
        println!("{:#?}", imu_reading);
    }
    Ok(())
}
