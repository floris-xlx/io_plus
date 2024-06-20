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

use serde_json::{ Value, json };

use std::time::Instant;

use io_plus::read_dir;

#[tokio::main]
async fn main() {
    let directory: &str = "G:\\";

    let files: Value = read_dir(directory).await;

    println!("{:#?}", files);


}



