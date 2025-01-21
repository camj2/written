use round::round;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();

    for drive in args.iter().skip(1) {
        let drive_stats: String =
            read_to_string(format!("/sys/block/{}/stat", drive)).expect("error");

        let hw_sector_size: String =
            read_to_string(format!("/sys/block/{}/queue/hw_sector_size", drive)).expect("error");

        let sector_size: f64 = hw_sector_size.trim().parse().expect("error");

        let mut drive_stat_index: Vec<f64> = Vec::new();
        for stat in drive_stats.split_whitespace() {
            drive_stat_index.push(stat.trim().parse().expect("error"));
        }

        let sectors_written: f64 = drive_stat_index[6];

        let total_written: f64 = sectors_written * sector_size;

        let (written, written_unit): (f64, &str) = if total_written < 1000.0 {
            (round(total_written, 2), "B")
        } else if total_written < 1048576.0 {
            (round(total_written / 1000.0, 2), "KB")
        } else if total_written < 1073741824.0 {
            (round(total_written / 1000.0 / 1000.0, 2), "MB")
        } else {
            (round(total_written / 1000.0 / 1000.0 / 1000.0, 2), "GB")
        };

        println!("{:.2} {}", written, written_unit);
    }
}
