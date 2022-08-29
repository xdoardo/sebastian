pub(crate) mod ariel;
pub(crate) mod time;
pub(crate) mod unimia;

use directories::ProjectDirs;
use lazy_static::lazy_static;
use sebastian_core::{ariel::ArielUserConfig, time::TimeTableConfig, unimia::UnimiaUserConfig};

lazy_static! {
    static ref CONFIG_DIR: String = ProjectDirs::from("", "", "sebastian")
        .unwrap()
        .config_dir()
        .to_string_lossy()
        .to_string();
    static ref CONFIG_PATH: String = ProjectDirs::from("", "", "sebastian")
        .unwrap()
        .config_dir()
        .join("config")
        .to_string_lossy()
        .to_string();
    static ref CURRENT_DIR: String = std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct UserConfig {
    pub ariel: Option<ArielUserConfig>,
    pub time: Option<TimeTableConfig>,
    pub mia: Option<UnimiaUserConfig>,
}

/// Access UniMi via CLI.
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub(crate) struct App {
    /// The path for the configuration.
    #[clap(short, long, default_value = &CONFIG_PATH, global = true)]
    pub config_path: String,

    /// Whether to save the current config or not.
    #[clap(short, long, global = true)]
    pub save: bool,

    /// Suppress every prompt and use the default answer.
    #[clap(long, long, global = true)]
    pub silent: bool,

    #[clap(subcommand)]
    pub action: Command,
}

impl App {
    pub fn to_config(&self) -> AppConfig {
        AppConfig {
            config_path: self.config_path.clone(),
            save: self.save,
            silent: self.silent,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AppConfig {
    pub config_path: String,

    pub save: bool,

    pub silent: bool,
}

#[derive(clap::Parser, Debug)]
pub(crate) enum Command {
    Ariel(ariel::Ariel),
    Time(time::Time),
    Unimia(unimia::Unimia),
}
