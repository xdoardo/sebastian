use super::{Ariel, OUTPUT_DIR};
use std::time::Duration;

/// Search course pages to scrape.
#[derive(clap::Parser, Clone, Debug)]
pub(crate) struct Search {
    /// The identifier of the course to scrape.
    pub name: String,
}

impl Ariel {
    pub(crate) fn search(&mut self, name: String) -> anyhow::Result<()> {
        log::info!("search '{}'", name);
        if self.nav.is_none() || self.user_config.is_none() {
            println!("{:?}", self);
            anyhow::bail!("cannot use uninitialized subcommand!")
        };

        let pb = indicatif::ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_style(
            indicatif::ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
                .unwrap()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
        );
        pb.set_message(format!("searching courses for '{}'...", name.clone()));

        let pages = self.nav.as_mut().unwrap().search(name.as_str())?;
        let pages = pages.iter().filter(|p| p.can_access).collect::<Vec<_>>();

        pb.set_style(
            indicatif::ProgressStyle::with_template("{prefix:.bold.dim} {wide_msg}").unwrap(),
        );
        if pages.len() == 0 {
            pb.set_style(
                indicatif::ProgressStyle::with_template("{prefix:.bold.dim} {wide_msg}").unwrap(),
            );
            pb.set_prefix("x");
            pb.finish_with_message(format!("no results found for query '{}'!", name));
            return Ok(());
        } else {
            pb.set_style(indicatif::ProgressStyle::with_template("").unwrap());
            pb.finish();
        }
        let ans = inquire::MultiSelect::new("Select the courses to search:", pages).prompt()?;
        if ans.len() == 0 {
            println!("No course selected!");
            return Ok(());
        }
        let action = inquire::Select::new("Select action", vec!["scrape", "print"]).prompt()?;
        if action == "scrape" {
            for page in ans {
                self.scrape(true, OUTPUT_DIR.to_string(), page.url.to_string())?;
            }
        } else {
        }

        Ok(())
    }
}
