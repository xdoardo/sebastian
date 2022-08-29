use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use app::AppConfig;
use clap::Parser;

mod app;

fn main() -> anyhow::Result<()> {
    let app = app::Sebastian::parse();
    let mut config_path = Path::new(&app.config_path);
    let mut save = app.save;
    let silent = app.silent;
    let action = app.action;

    let mut config = if config_path.exists() {
        toml::from_str(&fs::read_to_string(config_path)?)?
    } else {
        AppConfig {
            user: None,
            time: None,
            mia: None,
        }
    };

    match action {
        app::CommandDomain::Ariel(mut a) => match a.action {
            app::ariel::ArielAction::Scrape {
                interactive,
                ref output,
                ref path,
            } => {
                if let None = config.user {
                    a.config = Some(a.login(None, None)?);
                } else {
                    a.config = config.user.clone();
                };
                a.scrape(interactive, output.clone(), path.clone())?
            }
            app::ariel::ArielAction::Login {
                ref username,
                ref password,
            } => {
                config.user = Some(a.login(username.clone(), password.clone())?);
                save = true;
            }
            app::ariel::ArielAction::Search { ref name } => {
                if let None = config.user {
                    config.user = Some(a.login(None, None)?);
                    a.config = config.user.clone();
                } else {
                    a.config = config.user.clone();
                };
                a.search(&name)?
            }
        },
        app::CommandDomain::Time(mut t) => {
            t.config = config.time.clone();
            match t.action {
                app::time::TimeTableAction::Init => {
                    config.time = Some(t.init()?);
                    save = true;
                }

                app::time::TimeTableAction::Show => t.show()?,
            }
        }
        app::CommandDomain::Unimia(mut u) => {
            u.config = config.mia.clone();
            match u.action {
                app::unimia::UnimiaAction::Init => {
                    config.mia = Some(u.init()?);
                    save = true;
                }

                app::unimia::UnimiaAction::Show(s) => s.show()?,
            }
        }
    }

    if save {
        let maybe_path;

        if !silent {
            let mut path = inquire::Text::new("save to config in path:");
            path.default = config_path.to_str();
            maybe_path = path.prompt()?;
            config_path = Path::new(&maybe_path);
        }

        if !config_path.exists() {
            fs::create_dir_all(config_path.parent().unwrap())?
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
