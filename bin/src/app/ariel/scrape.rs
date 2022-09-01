use std::{fmt::Write, thread, time::Duration};

use super::Ariel;
use crate::app::CURRENT_DIR;
use sebastian_core::ariel::map::ArielSitemap;

lazy_static::lazy_static! {
    static ref OUTPUT_DIR: String = {
        let mut c = CURRENT_DIR.clone();
        c.push_str("/result");
        c
    };
    static ref ARIEL_SITEMAP: ArielSitemap = ArielSitemap::default();
}

/// Perform scraping.
#[derive(clap::Parser, Clone, Debug)]
pub(crate) struct Scrape {
    /// Perform scraping of Ariel website interactively.
    #[clap(short, long)]
    pub auto: bool,

    /// The base of the directory to save the results.
    #[clap(short, long, default_value = &OUTPUT_DIR)]
    pub output: String,

    /// The URL of the page to start the scraping from.
    #[clap(default_value = &ARIEL_SITEMAP.home_page_url)]
    pub url: String,
}

impl Ariel {
    pub(crate) async fn scrape(
        &mut self,
        auto: bool,
        out_path: String,
        url: String,
    ) -> anyhow::Result<()> {
        let page = self
            .nav
            .as_mut()
            .unwrap()
            .page_from_url(url.clone())
            .await?;
        log::debug!("page: {:?}", page);
        let mut to_ask = page.get_data();

        if auto {
            let mut stack = self.nav.as_mut().unwrap().get_children(page).await;

            while stack.len() != 0 {
                let child_page = stack.pop().unwrap();
                log::info!("getting data from child {}", child_page.url);
                to_ask.append(&mut child_page.get_data());
                stack.append(&mut self.nav.as_mut().unwrap().get_children(child_page).await);
            }
        } else {
            let mut stack = vec![];
            let pb = indicatif::ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(120));
            pb.set_style(
                indicatif::ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
                    .unwrap()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
            );
            pb.set_message(format!("searching pages from {}...", url));

            let children = self.nav.as_mut().unwrap().get_children(page).await;

            pb.set_style(indicatif::ProgressStyle::with_template("").unwrap());
            pb.finish();

            if !children.is_empty() {
                stack = inquire::MultiSelect::new("select pages to follow", children).prompt()?;
            }

            while stack.len() != 0 {
                let child_page = stack.pop().unwrap();
                log::info!("getting data from child {}", child_page.url);
                to_ask.append(&mut child_page.get_data());

                let pb = indicatif::ProgressBar::new_spinner();
                pb.enable_steady_tick(Duration::from_millis(120));
                pb.set_style(
                    indicatif::ProgressStyle::with_template(
                        "{prefix:.bold.dim} {spinner} {wide_msg}",
                    )
                    .unwrap()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
                );
                pb.set_message(format!("searching pages from {}...", child_page.url));
                let children = self.nav.as_mut().unwrap().get_children(child_page).await;
                pb.set_message(format!("searching pages from {}...", url));
                pb.set_style(indicatif::ProgressStyle::with_template("").unwrap());

                pb.finish();

                if children.is_empty() {
                    continue;
                }
                stack.append(
                    &mut inquire::MultiSelect::new("select pages to follow", children).prompt()?,
                );
            }
        }

        if to_ask.is_empty() {
            anyhow::bail!("Found no data to scrape!")
        }

        let selected = inquire::MultiSelect::new("Select data to scrape: ", to_ask).prompt()?;

        if selected.len() == 0 {
            anyhow::bail!("No data selected!")
        }

        let ticks = ["🌍 ", "🌎 ", "🌏 "];
        let progs = "█▓▒░  ";
        let sized_bar_style = indicatif::ProgressStyle::with_template(
            "{spinner} [{elapsed_precise}] {msg} [{bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .with_key(
            "eta",
            |state: &indicatif::ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            },
        )
        .progress_chars(progs)
        .tick_strings(&ticks);

        let unsized_bar_style = indicatif::ProgressStyle::with_template(
            "{spinner} [{elapsed_precise}] {msg} {total_bytes}",
        )
        .unwrap()
        .progress_chars(progs)
        .tick_strings(&ticks);

        let mut chunk_done_size_cx;
        let mut chunk_done_size_px;

        for d in selected {
            let size = self.nav.as_mut().unwrap().get_size(&d).await?;
            let style = if size != 0 {
                sized_bar_style.clone()
            } else {
                unsized_bar_style.clone()
            };
            let name = d.get_name();

            (chunk_done_size_px, chunk_done_size_cx) = std::sync::mpsc::channel::<u64>();

            let c = thread::spawn(move || {
                let pb = indicatif::ProgressBar::new(size);
                let mut chunk_bytes: u64 = 0;

                pb.set_style(style.clone());
                pb.set_position(0);
                pb.set_message(format!("fetch {}", name));

                for bs in chunk_done_size_cx {
                    chunk_bytes += bs;
                    pb.set_position(chunk_bytes.try_into().unwrap());
                    pb.set_length(chunk_bytes)
                }

                pb.finish_with_message(format!("downloaded {}", name));
            });

            self.nav
                .as_mut()
                .unwrap()
                .download(out_path.clone(), d.clone(), chunk_done_size_px)
                .await?;

            c.join().expect("child panicked");
        }

        Ok(())
    }
}
