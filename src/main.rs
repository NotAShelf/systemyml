#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
extern crate serde;

use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
mod service_struct;

#[derive(Debug, Deserialize, Serialize)]
struct ServiceConfig {
    pub name: String,
    pub install: Option<HashMap<String, String>>,
    pub unit: Option<HashMap<String, String>>,
    pub service: Option<HashMap<String, String>>,
    pub environment: Option<HashMap<String, String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("systemyml")
        .version("0.0.1")
        .author("NotAShelf")
        .about("Creates systemd service units from YAML configuration files.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("PATH")
                .help("Specify the directory or YAML file containing service configuration.")
                .default_value("services") // Default value for the config directory.
                .takes_value(true),
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("DIRECTORY")
                .help("Specify the target directory for systemd service units.")
                .default_value("/etc/systemd/system") // Default value for the target directory.
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Enable verbose logging."),
        )
        .arg(
            Arg::with_name("stdout")
                .long("stdout")
                .help("Print the generated service config to stdout."),
        )
        .arg(
            Arg::with_name("dryrun")
                .long("dryrun")
                .help("Describe what would happen without making changes."),
        )
        .arg(
            Arg::with_name("safe")
                .long("safe")
                .help("Copy service units to the target directory without starting services."),
        )
        .get_matches();

    if matches.is_present("verbose") {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    info!("Starting the program.");

    let config_path = Path::new(matches.value_of("config").unwrap());
    let target_dir = PathBuf::from(matches.value_of("target").unwrap());

    if config_path.is_dir() {
        for entry in fs::read_dir(&config_path)? {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_file() && file_path.extension().map_or(false, |e| e == "yaml") {
                    if let Err(err) = process_service_config(
                        &file_path,
                        &target_dir,
                        matches.is_present("stdout"),
                        matches.is_present("dryrun"),
                        matches.is_present("safe"),
                    ) {
                        error!("Failed to process service configuration: {}", err);
                    }
                }
            }
        }
    } else if config_path.is_file() {
        if let Err(err) = process_service_config(
            config_path,
            &target_dir,
            matches.is_present("stdout"),
            matches.is_present("dryrun"),
            matches.is_present("safe"),
        ) {
            error!("Failed to process service configuration: {}", err);
        }
    } else {
        error!("Invalid configuration path: {:?}", config_path);
    }

    info!("Program completed successfully.");

    Ok(())
}

/// Process the service configuration from a YAML file and create systemd service units.
///
/// # Arguments
///
/// * `file_path` - The path to the YAML configuration file.
/// * `target_dir` - The target directory where systemd service units will be installed.
/// * `print_to_stdout` - Whether to print the generated service config to stdout.
/// * `dryrun` - Whether to describe what would happen without making changes.
/// * `safe` - Whether to copy service units to the target directory without starting services.
///
/// # Returns
///
/// A Result indicating success or an error.
fn process_service_config(
    file_path: &std::path::Path,
    target_dir: &std::path::Path,
    _print_to_stdout: bool,
    dryrun: bool,
    safe: bool,
) -> Result<(), Box<dyn Error>> {
    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?;
    }

    let yaml_content = fs::read_to_string(file_path)?;
    let service_config: HashMap<String, ServiceConfig> = serde_yaml::from_str(&yaml_content)?;

    for (name, config) in &service_config {
        let serialized_config = serde_yaml::to_string(&config)?;

        let unit_filename = format!("{}.service", name);
        let unit_path = target_dir.join(&unit_filename);

        info!("unit_filename: {:?}", unit_filename);
        info!("unit_path: {:?}", unit_path);

        if dryrun {
            println!(
                "Generated service config for '{}':\n\n{}",
                name, serialized_config
            );
            println!("Dry run: Would create systemd unit: {:?}", unit_path);

            if safe {
                println!("Dry run: Would enable service: {}", name);
                println!("Dry run: Would start service: {}", name);
            }
        } else {
            let mut unit_file = File::create(&unit_path)?;
            unit_file.write_all(serialized_config.as_bytes())?;

            if safe {
                info!("Copied systemd unit to target directory: {:?}", unit_path);
            } else {
                if let Err(err) = enable_and_start_service(&unit_filename) {
                    error!("Failed to enable and start service '{}': {}", name, err);
                } else {
                    info!("Enabled and started service: {}", name);
                }
            }
        }
    }

    Ok(())
}

/// Enable and start a systemd service by its name.
///
/// # Arguments
///
/// * `service_name` - The name of the systemd service to enable and start.
///
/// # Returns
///
/// A Result indicating success or an error.
fn enable_and_start_service(service_name: &str) -> Result<(), Box<dyn Error>> {
    // TODO: make this do something
    println!("Enabled and started service: {}", service_name);
    Ok(())
}
