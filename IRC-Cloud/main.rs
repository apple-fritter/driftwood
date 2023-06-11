use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Provide the path to the IRC log file
    let log_file_path = "path/to/log_file.txt";

    // Open the log file
    let file = File::open(log_file_path).expect("Failed to open log file");
    let reader = BufReader::new(file);

    // Variables to store message details
    let mut current_channel = String::new();
    let mut current_nick = String::new();
    let mut is_private_message = false;

    // Iterate over each line in the log file
    for line in reader.lines() {
        if let Ok(line) = line {
            // Skip the header line if it exists
            if line.starts_with('#') || line.starts_with('<') {
                continue;
            }

            // Extract the timestamp, sender nickname (if applicable), and message content
            if let Some((timestamp, sender, message)) = extract_message(&line) {
                if line.starts_with("→ Joined channel") {
                    // Extract the channel name from the line
                    current_channel = extract_channel_name(&line);
                    is_private_message = false;
                } else if line.starts_with('*') {
                    // Extract the sender nickname from the line
                    current_nick = extract_sender_nick(&line);
                    is_private_message = false;
                } else if line.starts_with('<') {
                    // Extract the sender nickname from the line
                    current_nick = extract_sender_nick(&line);
                    is_private_message = true;
                }

                // Process the extracted message
                if is_private_message {
                    // Handle private messages
                    process_private_message(&current_nick, &timestamp, &message);
                } else {
                    // Handle channel messages
                    process_channel_message(&current_channel, &timestamp, &sender, &message);
                }
            }
        }
    }
}

fn extract_message(line: &str) -> Option<(String, String, String)> {
    // Modify the timestamp pattern according to your actual timestamp format
    if let Some(timestamp_end) = line.find("]") {
        let timestamp = line[1..timestamp_end].to_string();
        let message = line[(timestamp_end + 2)..].to_string();

        // Check if the line contains a sender nickname
        if let Some(sender_end) = message.find("> ") {
            let sender = message[1..sender_end].to_string();
            let message_content = message[(sender_end + 2)..].to_string();
            Some((timestamp, sender, message_content))
        } else {
            Some((timestamp, String::new(), message))
        }
    } else {
        None
    }
}

fn extract_channel_name(line: &str) -> String {
    // Extract the channel name from the line
    let channel_start = line.find('#').unwrap_or(0);
    let channel_end = line.find(' ').unwrap_or(line.len());
    line[channel_start..channel_end].to_string()
}

fn extract_sender_nick(line: &str) -> String {
    // Extract the sender nickname from the line
    let nick_end = line.find('>').unwrap_or(line.len());
    line[1..nick_end].to_string()
}

fn process_private_message(nick: &str, timestamp: &str, message: &str) {
    // Process private messages (e.g., save to a separate file)
    println!("Private Message: [{}][{}]: {}", timestamp, nick, message);
}

fn process_channel_message(channel: &str, timestamp: &str, sender: &str, message: &str) {
    // Process channel messages (e.g., save to a separate file)
    println!("Channel Message: [{}][{}@{}]: {}", timestamp, sender, channel, message);
}
