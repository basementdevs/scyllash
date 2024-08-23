use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

static DEFAULT_SETTINGS_STR: &str =
  include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/default_config.toml"));

static SETTINGS_PATH: &str = concat!(env!("HOME"), "/.config/scyllash");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
  #[command(subcommand)]
  pub command: Option<Command>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
pub enum Command {
  /// Starts ScyllaSH with a provided connection
  Run {
    #[command(flatten)]
    connection: ConnectionConfig,
  },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Args, Deserialize, Serialize)]
pub struct ConnectionConfig {
  /// Scylla Host
  #[arg(long, default_value_t = get_default_string_value("hostname"))]
  pub hostname: String,
  /// Scylla User
  #[arg(short, long, default_value_t = get_default_string_value("username"))]
  pub username: String,

  /// Scylla Password
  #[arg(short, long, default_value_t = get_default_string_value("password"))]
  pub password: String,

  /// Scylla Timeout
  #[arg(short, long, default_value_t = 5)]
  pub timeout: u64,
}

fn get_default_string_value(key: &str) -> String {
  let config = ConfigToml::default();
  let connection = config.connection.clone();
  match key {
    "hostname" => {
      if !config.connection.hostname.is_empty() {
        connection.hostname
      } else {
        String::from("localhost:9042")
      }
    }
    "username" => {
      if !config.connection.username.is_empty() {
        connection.username
      } else {
        String::from("scylla")
      }
    }
    "password" => {
      if !config.connection.password.is_empty() {
        connection.password
      } else {
        String::from("")
      }
    }
    _ => String::from(""),
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigToml {
  pub connection: ConnectionConfig,
}

impl Default for ConfigToml {
  fn default() -> Self {
    let settings_path = Path::new(SETTINGS_PATH);
    let file_path = settings_path.join("config.toml");

    if !settings_path.exists() {
      println!("No settings directory identified");
      fs::create_dir_all(settings_path).expect("Failed to create settings directory");
    }

    if !file_path.exists() {
      println!("No config file identified");
      let mut file = File::create(&file_path).expect("Failed to create settings file");
      file
        .write_all(DEFAULT_SETTINGS_STR.as_bytes())
        .expect("Failed to write default settings");
    }

    let toml_str = fs::read_to_string(&file_path).expect("Failed to read settings file");
    let cargo_toml: ConfigToml =
      toml::from_str(&toml_str).expect("Failed to deserialize Cargo.toml");

    cargo_toml
  }
}
