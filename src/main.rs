use std::{fs::File, io::Write, thread, time::Duration};

use chrono::{Local, Utc};
use sysinfo::{ProcessRefreshKind, System};

#[derive(Debug)]
struct ProcInfo {
    pid: u32,
    physical_memory: u64,
    virutal_memory: u64,
}

impl ProcInfo {
    fn as_csv_line(&self, timestamp: i64) -> String {
        format!(
            "{},{},{},{}\n",
            self.pid, timestamp, self.physical_memory, self.virutal_memory
        )
    }
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let process_name = match args.next() {
        Some(name) => name.to_string(),
        None => {
            println!("Missing process_name cli argument");
            return;
        }
    };
    let interval = match args.next() {
        Some(number) => match number.parse::<u64>() {
            Ok(n) => n,
            Err(_) => {
                println!("Failed to parse interval {}. Using default 20s", number);
                20
            }
        },
        None => 20,
    };
    let mut system = System::new();
    system.refresh_processes_specifics(
        sysinfo::ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::everything(),
    );
    let now = Local::now();
    let fmt_time = now.format("%Y-%m-%d_%H:%M");
    let file_name = format!("{}_{}.csv", process_name, fmt_time);
    let mut output_file = File::create(file_name).expect("Failed to create output_file");
    let headers = "pid,timestamp,ram,vram\n";
    output_file.write(headers.as_bytes()).unwrap();
    let start_time = now.format("%H:%M:%S");
    println!(
        "[{}] Measuring the memory usage of process: `{}` every {} seconds...",
        start_time, process_name, interval
    );
    loop {
        thread::sleep(Duration::from_secs(interval));
        system.refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::everything(),
        );
        let procs: Vec<ProcInfo> = system
            .processes()
            .iter()
            .filter_map(|(pid, process)| {
                if process.name().to_string_lossy() != process_name {
                    None
                } else {
                    Some(ProcInfo {
                        pid: pid.as_u32(),
                        physical_memory: process.memory() / 1024,
                        virutal_memory: process.virtual_memory() / 1024,
                    })
                }
            })
            .collect();
        if procs.is_empty() {
            println!("No process found with name: {}", process_name);
        } else {
            let now = Utc::now();
            for proc in procs.iter() {
                let line = proc.as_csv_line(now.timestamp());
                output_file.write(line.as_bytes()).unwrap();
            }
        }
    }
}
