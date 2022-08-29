use std::time::Duration;

use sebastian_core::ariel::{ArielNavigator, ArielUserConfig};

use super::Ariel;

/// Log into Ariel and initialize the configuration.
#[derive(clap::Parser, Clone, Debug)]
pub(crate) struct Login {
    #[clap(short, long)]
    pub username: Option<String>,

    #[clap(short, long)]
    pub password: Option<String>,
}

impl Ariel {
    pub(crate) fn login(
        &mut self,
        username: Option<String>,
        password: Option<String>,
        auto: bool,
    ) -> anyhow::Result<()> {
        log::debug!("username: {:?}, password: {:?}", username, password);
        let mut username_prompt = inquire::Text::new("username:");
        fn suggester(str: &str) -> Result<Vec<String>, inquire::CustomUserError> {
            let email_domain_regex = regex::Regex::new(r"@.*")?;
            let mails = ["@studenti.unimi.it", "@unimi.it"];

            let caps = if let Some(caps) = email_domain_regex.captures(str) {
                caps
            } else {
                return Ok(vec![]);
            };
            let domain = if let Some(domain) = caps.get(0) {
                domain.as_str()
            } else {
                return Ok(vec![]);
            };

            let mut ret = vec![];
            for mail in mails {
                if mail.starts_with(domain) {
                    ret.push(format!("{}{}", str, mail.strip_prefix(domain).unwrap()));
                }
            }
            Ok(ret)
        }
        username_prompt.suggester = Some(&suggester);

        let username = if let Some(username) = username {
            if auto {
                username
            } else {
                username_prompt.default = Some(&username);
                username_prompt.prompt()?
            }
        } else {
            username_prompt.prompt()?
        };

        let password = if let Some(password) = password && auto{
            password
        } else {
            inquire::Password::new("password:").prompt()?
        };

        let config = ArielUserConfig { username, password };
        self.nav = Some(ArielNavigator::new(config.clone()));

        let pb = indicatif::ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_style(
            indicatif::ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
                .unwrap()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
        );
        pb.set_message("logging in...");
        let outcome = self.nav.as_mut().unwrap().login();
        match outcome {
            Ok(_) => {
                pb.set_style(
                    indicatif::ProgressStyle::with_template("{prefix:.bold.dim} {wide_msg}")
                        .unwrap(),
                );
                pb.set_prefix("✓");
                pb.finish_with_message("logged in!");
                self.user_config = Some(config);
                Ok(())
            }
            Err(e) => {
                pb.set_style(indicatif::ProgressStyle::with_template("").unwrap());

                pb.finish();
                anyhow::bail!("could not login with supplied username and password, {}", e)
            }
        }
    }
}
