use bytesize::ByteSize;
use written::ProcDiskStats;

fn main() {
    for block_device in ProcDiskStats::collect() {
        if block_device.is_loop() {
            continue;
        }
        if let Some(bytes) = block_device.bytes_written() {
            println!("{}: {}", block_device.device_name, ByteSize(bytes));
        }
    }
}
