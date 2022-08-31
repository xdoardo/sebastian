use std::{fs::OpenOptions, io::Write, path::Path};

use app::{Command, UserConfig};
use clap::Parser;
mod app;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::trace!("starting app!");
    let app = app::App::parse();
    let app_config = app.to_config();
    let mut config_path = Path::new(&app.config_path);
    log::trace!("config path is {:?}", config_path);

    let mut config = if config_path.exists() {
        toml::from_str(&std::fs::read_to_string(config_path)?)?
    } else {
        UserConfig {
            ariel: None,
            time: None,
            mia: None,
        }
    };

    log::trace!("config is {:?}", config);

    let app_config = match app.action {
        Command::Ariel(mut a) => {
            log::debug!("subcommand is ariel");
            let (app_config, ariel_config) = a.run(app_config, config.ariel).await?;
            config.ariel = Some(ariel_config);
            app_config
        }
        Command::Time(mut t) => {
            log::debug!("subcommand is time");
            let (app_config, time_config) = t.run(app_config, config.time).await?;
            config.time = Some(time_config);
            app_config
        }
        Command::Unimia(mut u) => {
            log::debug!("subcommand is mia");
            let (app_config, mia_config) = u.run(app_config, config.mia).await?;
            config.mia = Some(mia_config);
            app_config
        }
    };

    if app_config.save {
        log::debug!("saving config");
        let maybe_path;
        if !app_config.silent {
            let mut path = inquire::Text::new("save to config in path:");
            path.default = config_path.to_str();
            maybe_path = path.prompt()?;
            config_path = Path::new(&maybe_path);
        }

        log::debug!("config path: {:?}", config_path);

        if !config_path.exists() {
            log::trace!("creating path {:?}", config_path);
            std::fs::create_dir_all(config_path.parent().unwrap())?
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_path)?;

        file.write_all(toml::to_string(&config)?.as_bytes())?;
    }
    Ok(())
}
