use clap::Parser;

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

pub fn run() {
    let args = Args::parse();
    println!("{}", args.file.to_string_lossy());
}
