use std::fs;
// use std::os::unix::fs;s

use std::collections::BTreeMap;

use std::env;
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};
// round crate or build own solution
// bytes to mb crate or build own solution

// https://www.kernel.org/doc/html/latest/admin-guide/iostats.html

pub struct ProcDiskStats {
    pub major_device_number: u64, // Field Prefix 1
    pub minor_device_number: u64, // Field Prefix 2
    pub device_name: String,        // Field Prefix 3
    pub stats: DiskStats,           // Field 1-17
}

pub struct DiskStats {
    pub reads_completed: u64,                      // Field 1
    pub reads_merged: u64,                         // Field 2
    pub sectors_read: u64,                         // Field 3
    pub milliseconds_spent_reading: u64,           // Field 4
    pub writes_completed: u64,                     // Field 5
    pub writes_merged: u64,                        // Field 6
    pub sectors_written: u64,                      // Field 7
    pub milliseconds_spent_writing: u64,           // Field 8
    pub io_currently_in_progress: u64,             // Field 9
    pub milliseconds_spent_doing_io: u64,          // Field 10
    pub weighted_milliseconds_spent_doing_io: u64, // Field 11
    pub discards_completed: u64,                   // Field 12
    pub discards_merged: u64,                      // Field 13
    pub sectors_discarded: u64,                    // Field 14
    pub milliseconds_spent_discarding: u64,        // Field 15
    pub flush_requests_completed: u64,             // Field 16
    pub milliseconds_spent_flushing: u64,          // Field 17
}

pub struct SectorSize {
    logical_block_size: u64,
    physical_block_size: u64,
}

impl SectorSize {
    pub fn from_name<T: fmt::Display>(device: T) -> Option<Self> {
        let path = format!("/sys/block/{device}/queue/logical_block_size");
        let file_contents = fs::read_to_string(&path).ok()?;
        let logical_block_size: u64 = file_contents.trim().parse().ok()?;

        let path = format!("/sys/block/{device}/queue/physical_block_size");
        let file_contents = fs::read_to_string(&path).ok()?;
        let physical_block_size: u64 = file_contents.trim().parse().ok()?;

        Some(Self {
            logical_block_size,
            physical_block_size,
        })
    }
}

impl DiskStats {
    pub fn from(line: &[&str]) -> Self {
        let mut fields: Vec<u64> = Vec::new();

        for f in line {
            let field: u64 = f.trim().parse().expect("Value should be an integer.");

            fields.push(field);
        }

        if fields.len() != 17 {
            panic!("Line should have 17 fields.");
        }

        Self {
            reads_completed: fields[0],
            reads_merged: fields[1],
            sectors_read: fields[2],
            milliseconds_spent_reading: fields[3],
            writes_completed: fields[4],
            writes_merged: fields[5],
            sectors_written: fields[6],
            milliseconds_spent_writing: fields[7],
            io_currently_in_progress: fields[8],
            milliseconds_spent_doing_io: fields[9],
            weighted_milliseconds_spent_doing_io: fields[10],
            discards_completed: fields[11],
            discards_merged: fields[12],
            sectors_discarded: fields[13],
            milliseconds_spent_discarding: fields[14],
            flush_requests_completed: fields[15],
            milliseconds_spent_flushing: fields[16],
        }
    }

    // pub fn from_file(path: &Path) -> Self {}
}

// fn parse(int: &str) -> u64 {
//     int.trim().parse().expect("Value should be an integer.")
// }

impl ProcDiskStats {
    // pub fn from(fields: &[str]) -> Self {}

    pub fn get() -> Vec<Self> {
        let file_contents = fs::read_to_string("/proc/diskstats").expect("File should exist.");

        let mut collection: Vec<Self> = Vec::new();

        for line in file_contents.lines() {
            let fields: Vec<&str> = line.split_whitespace().into_iter().collect();

            if fields.len() != 20 {
                panic!("Line should have 20 fields.");
            }

            let entry = Self {
                major_device_number: fields[0].trim().parse().unwrap(),
                minor_device_number: fields[1].trim().parse().unwrap(),
                device_name: fields[2].to_string(),
                stats: DiskStats::from(&fields[3..]),
            };
            collection.push(entry);
        }
        collection
    }

    pub fn data_written(&self) -> Option<u64> {
        let sector_size = SectorSize::from_name(&self.device_name)?;

        let disk_write_total: u64 = self.stats.sectors_written * sector_size.logical_block_size;

        Some(disk_write_total)

        // println!("{}: {}", &self.device_name, disk_write_total);
    }

    // pub fn from<D: Display>(device: D) -> Option<Self> {
    //     let device = device.to_string(); // nvme0n1

    //     let mut disk_stats: Vec<String> = Vec::new();

    //     for line in file_contents.lines() {
    //         let stats = DiskStats::from(&line);

    //         // let stats =

    //         for field in line.split_whitespace() {
    //             // disk_stats.push
    //         }
    //     }

    //     Some(Self { disk, bytes })
    // }

    // pub fn disk(&self) -> String {
    //     self.disk.to_string()
    // }

    // pub fn bytes(&self) -> u64 {
    //     self.bytes
    // }
}

// impl fmt::Display for Config {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut first = true;

//         for (id, peer) in &self.peers {
//             if !first {
//                 write!(f, "\n")?;
//             }
//             first = false;

//             write!(f, "{}: {}", id, peer.name)?;
//         }
//         Ok(())
//     }
// }
