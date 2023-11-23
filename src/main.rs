use chrono::Duration;
use chrono::{DateTime, Local, Utc};
use colored::Colorize;
use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let slug = &args[1];
    let data = load_file(slug);
    print_output(slug, data);
}

fn load_file(slug: &String) -> Vec<DataPoint> {
    let file_path = format!("./data/{slug}.txt");
    println!("{}", format!("Reading file {file_path}").dimmed());

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut result = Vec::new();

    for line in contents.split("\n") {
        println!("{}", format!("  {line}").dimmed());
        let parts: Vec<&str> = line.split(", ").collect();
        if parts.len() == 2 {
            let count = parts[0].parse::<i32>().unwrap();
            let timestamp = parts[1].parse::<DateTime<Utc>>().expect("Invalid DateTime");
            let data_point = DataPoint {
                timestamp: timestamp,
                count: count,
            };
            result.push(data_point);
        }
    }
    return result;
}

fn print_output(slug: &String, data: Vec<DataPoint>) {
    let first_item = &data[0];
    let last_item = &data[data.len() - 1];
    let total_duration = last_item
        .timestamp
        .signed_duration_since(first_item.timestamp);
    let processed_count = first_item.count - last_item.count;
    let percentage = (processed_count as f32) / (first_item.count as f32) * 100.0;

    let remaining_count = last_item.count;

    // terrible algorithm is terrible
    let eta_seconds =
        (total_duration.num_seconds() * (remaining_count as i64)) / (processed_count as i64);
    let eta_duration = Duration::seconds(eta_seconds as i64);
    let eta_time = last_item.timestamp + eta_duration;

    println!("Job Name: \t{}", slug.blue().bold());
    println!("Start Count: \t{}", first_item.count.to_string().cyan());
    println!("Remaining: \t{}", remaining_count.to_string().cyan());
    println!("Progress: \t{}%", percentage.to_string().cyan());
    println!("");
    println!("Elapsed: \t{}", pretty_duration(total_duration).cyan());
    println!("Remaining: \t{}", pretty_duration(eta_duration).cyan());
    println!(
        "Light at the Tunnel's End: {}",
        eta_time.with_timezone(&Local).to_string().green().bold()
    );
    println!("");
}

fn pretty_duration(duration: Duration) -> String {
    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if days > 0 || hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if days > 0 || hours > 0 || minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if days > 0 || hours > 0 || minutes > 0 || seconds > 0 {
        parts.push(format!("{}s", seconds));
    }

    return parts.join(" ");
}

struct DataPoint {
    timestamp: DateTime<Utc>,
    count: i32,
}
