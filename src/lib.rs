use std::path::PathBuf;
mod imu;
mod ui;

pub fn run(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let imu_readings: Vec<imu::IMUEntry> = imu::load_csv(path)?;
    let r_acceleration = imu::calculate_accelerations(&imu_readings);

    ui::run_tui(&r_acceleration)?;
    Ok(())
}
