use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

fn main() {
    // Input XChat log file path
    let input_path = std::env::args()
        .nth(1)
        .expect("Please provide the input XChat log file as an argument.");

    // Output directory
    let output_dir = Path::new("driftwood_logs");

    // Create the output directory if it doesn't exist
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).expect("Failed to create the output directory.");
    }

    // Read the input XChat log file
    let input_file = fs::File::open(input_path).expect("Failed to open the input file.");
    let reader = BufReader::new(input_file);

    // Process each log entry
    for line in reader.lines() {
        if let Ok(entry) = line {
            let fields: Vec<&str> = entry.split("☕").collect();
            if fields.len() >= 7 {
                let server = fields[4];
                let channel = ""; // Update with actual channel extraction
                let year = ""; // Update with actual year extraction
                let month = ""; // Update with actual month extraction
                let day = ""; // Update with actual day extraction
                let log_message = fields[5];

                // Construct the output file path
                let output_path = output_dir
                    .join(server)
                    .join(channel)
                    .join(year)
                    .join(month)
                    .join(day)
                    .with_extension("txt");

                // Create the necessary directories if they don't exist
                if let Some(parent_dir) = output_path.parent() {
                    fs::create_dir_all(parent_dir).expect("Failed to create the output directories.");
                }

                // Write the log entry to the output file
                let mut output_file =
                    fs::OpenOptions::new().append(true).create(true).open(output_path)
                        .expect("Failed to create or open the output file.");
                writeln!(output_file, "{}☕{}☕{}☕{}☕{}☕{}☕", "", fields[1], fields[2], fields[3], fields[4], log_message)
                    .expect("Failed to write to the output file.");
            }
        }
    }
}
