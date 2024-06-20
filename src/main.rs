#![allow(unused_imports)]

// legacy speed
// Total amount of directories: 131
// Total time taken: 356 ms


// core imports
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::io;

use serde_json::{Value, json};


use std::time::Instant;

#[tokio::main]
async fn main() {
    let directory: &str = "G:\\";
    let start_time = Instant::now();

    let (directories, files) = read_directory(directory).await;

    let total_time = start_time.elapsed().as_millis();
    let result = json!({
        "total_time_ms": total_time,
        "total_items": directories.len() + files.len(),
        "directories": directories,
        "files": files
    });

    println!("{:#?}", result);
}


async fn read_directory(directory: &str) -> (Vec<Value>, Vec<Value>) {
    let mut directories: Vec<Value> = Vec::new();
    let mut files: Vec<Value> = Vec::new();
    let mut cache: HashMap<String, (String, String)> = HashMap::new();

    match fs::read_dir(directory) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path: std::path::PathBuf = entry.path();
                        let path_str: String = path.to_string_lossy().to_string();
                        if path.is_dir() {
                            directories.push(json!({
                                "name": path.file_name().unwrap().to_string_lossy(),
                                "directory": path_str.clone(),
                                "drive": path_str.split("\\").collect::<Vec<&str>>()[0],
                            }));
                        } else {
                            let (file_name, extension) = cache.entry(path_str.clone()).or_insert_with(|| {
                                (
                                    path.file_name().unwrap().to_string_lossy().to_string(),
                                    path.extension().unwrap_or_default().to_string_lossy().to_string()
                                )
                            });
                            files.push(json!({
                                "name": file_name,
                                "directory": path_str,
                                "extension": extension,
                                "file_size_mb": (fs::metadata(&path).unwrap().len() as f64) / (1024.0 * 1024.0),
                            }));
                        }
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }

    (directories, files)
}
