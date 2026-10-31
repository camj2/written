use bytesize::ByteSize;
use written::ProcDiskStats;

fn main() {
    for stats in ProcDiskStats::get() {
        if let Some(bytes) = stats.data_written() {
            if bytes == 0 {
                continue;
            }
            println!("{}: {}", stats.device_name, ByteSize(bytes));
        }
    }
}
