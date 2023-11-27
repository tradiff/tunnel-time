mod data_file;

use chrono::Duration;
use chrono::Local;
use colored::Colorize;
use data_file::DataFile;
use std::env;
use std::fs;
use toml;

fn main() {
    let args: Vec<String> = env::args().collect();
    let slug = &args[1];
    let data = load_file(slug);
    print_output(slug, data);
}

fn load_file(slug: &String) -> DataFile {
    let file_path = format!("./data/{slug}.toml");
    println!("{}", format!("Reading file {file_path}").dimmed());

    let contents = fs::read_to_string(file_path).expect("Error reading the file");
    let data_file: DataFile = toml::from_str(&contents).expect("Error parsing the file");

    return data_file;
}

fn print_output(slug: &String, data_file: DataFile) {
    if &data_file.data_points.len() <= &1 {
        println!("Create some data and try again");
        return;
    }

    let first_item = &data_file.data_points.first().unwrap();
    let last_item = &data_file.data_points.last().unwrap();

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
    let job_name = data_file.job_name.unwrap_or(slug.to_string());

    println!("Job Name: \t{}", job_name.blue().bold());
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
