use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

const HOT_BEVERAGE: char = '☕';

struct IRCLogEntry {
    ignore: bool,
    timestamp: u64,
    nick: String,
    message: String,
}

fn parse_log_entry(line: &str) -> IRCLogEntry {
    let fields: Vec<&str> = line.split("\t").collect();
    let ignore = fields[0].trim() == "#";
    let timestamp: u64 = fields[1].parse().unwrap();
    let nick = String::from(fields[2]);
    let message = String::from(fields[3]);

    IRCLogEntry {
        ignore,
        timestamp,
        nick,
        message,
    }
}

fn read_log_file(file_path: &Path) -> Vec<IRCLogEntry> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| parse_log_entry(&line.unwrap()))
        .collect()
}

fn write_log_entry(
    log_entry: &IRCLogEntry,
    log_date_dir: &Path,
    log_file_path: &Path,
) -> std::io::Result<()> {
    if log_entry.ignore {
        return Ok(()); // ignore rows marked with #
    }

    let log_line = format!(
        "{}{}{}{}{}{}{}{}{}\n",
        HOT_BEVERAGE,
        log_entry.timestamp,
        HOT_BEVERAGE,
        log_entry.nick,
        HOT_BEVERAGE,
        log_entry.message,
        HOT_BEVERAGE,
        HOT_BEVERAGE,
    );

    let date_dir_path = log_date_dir.join(format!("{:02}", log_entry.timestamp % 100_000_000));
    fs::create_dir_all(&date_dir_path)?;

    let file = OpenOptions::new().create(true).append(true).open(log_file_path)?;

    writeln!(file, "{}", log_line)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("."));

    let weechat_log_dir = Path::new(&home_dir).join(".weechat").join("logs");

    let irc_log_dir = Path::new(&home_dir).join("irc-logs");

    fs::create_dir_all(&irc_log_dir)?;

    for channel_dir in fs::read_dir(&weechat_log_dir)? {
        let channel_dir = channel_dir?.path();

        if channel_dir.is_dir() {
            let channel_name = channel_dir.file_name().unwrap().to_string_lossy();

            for log_file in fs::read_dir(&channel_dir)? {
                let log_file_path = log_file?.path();
                let log_file_name = log_file_path.file_name().unwrap().to_string_lossy();

                if log_file_path.is_file() && log_file_name.ends_with(".weechatlog") {
                    let date_parts: Vec<&str> = log_file_name
                        .trim_end_matches(".weechatlog")
                        .split("-")
                        .collect();

                    if date_parts.len() == 3 {
                        let year_dir = irc_log_dir.join(date_parts[0]);
                        let month_dir = year_dir.join(date_parts[1]);
                        let day_dir = month_dir.join(date_parts[2]);

                        fs::create_dir_all(&day_dir)?;

                        let log_entries = read
                        let log_entries = read_log_file(&log_file_path);

                        for log_entry in log_entries {
                            let log_date_dir = day_dir.join(format!(
                                "{:02}",
                                log_entry.timestamp % 100_000_000
                            ));

                            write_log_entry(
                                &log_entry,
                                &log_date_dir,
                                &log_file_path,
                            )?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
