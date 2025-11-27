use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

struct AggregateInfo {
    log_counts: HashMap<LogLevel, usize>,
    skipped_files: Vec<String>,
}

fn aggregate_logs(dir: &Path) -> Result<AggregateInfo, Box<dyn Error>> {
    let mut log_counts: HashMap<LogLevel, usize> = HashMap::new();
    let mut skipped_files: Vec<String> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("log") {
            continue;
        }

        let file = match fs::File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Грешка при отваряне на файл {:?}: {}", path.display(), e);
                skipped_files.push(path.display().to_string());
                continue;
            }
        };

        let reader = BufReader::new(file);
        let mut file_ok = true;

        for (line_num, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    eprintln!(
                        "Грешка при четене на ред {} във файл {:?}: {}",
                        line_num,
                        path.display(),
                        e
                    );
                    file_ok = false;
                    break;
                }
            };

            let mut parts = line.split_whitespace();
            let _timestamp = parts.next();
            let level_str = match parts.next() {
                Some(s) => s,
                None => {
                    eprintln!(
                        "Невалиден формат във файл {:?}, ред {}",
                        path.display(),
                        line_num
                    );
                    file_ok = false;
                    break;
                }
            };

            let level = match level_str.to_ascii_uppercase().as_str() {
                "ERROR" => LogLevel::Error,
                "WARN" | "WARNING" => LogLevel::Warn,
                "INFO" => LogLevel::Info,
                "DEBUG" => LogLevel::Debug,
                _ => {
                    eprintln!(
                        "Непознато ниво '{}' във файл {:?}, ред {}",
                        level_str,
                        path.display(),
                        line_num
                    );
                    file_ok = false;
                    break;
                }
            };

            *log_counts.entry(level).or_insert(0) += 1;
        }

        if !file_ok {
            skipped_files.push(path.display().to_string());
        }
    }

    Ok(AggregateInfo {
        log_counts,
        skipped_files,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let dir = Path::new("./logs");
    let result = aggregate_logs(dir)?;

    println!("--- Резултати ---");
    for (level, count) in &result.log_counts {
        println!("{:?}: {}", level, count);
    }

    if !result.skipped_files.is_empty() {
        println!("\nПропуснати файлове:");
        for f in &result.skipped_files {
            println!(" - {}", f);
        }
    }
    Ok(())
}
