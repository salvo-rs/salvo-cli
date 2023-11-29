use anyhow::Result;
use clap::Parser;
mod test;
mod utils;
use i18n::set_locale;
use utils::check_for_updates;
mod i18n;
rust_i18n::i18n!("locales", fallback = "en");
#[derive(Parser, Debug)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Fankai liu <liufankai137@outlook.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    New(Project),
}
#[derive(Parser, Debug, Clone)]
pub struct Project {
    pub project_name: String,
    #[clap(short, long)]
    lang: Option<String>,
}
#[tokio::main]
async fn main() -> Result<()> {
    utils::print_logo();
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::New(project) => {
            set_locale(&project.lang);
            check_for_updates().await;
            match utils::create_project(project) {
                Ok(_) => (),
                Err(e) => utils::error(e.to_string()),
            };
        }
    }
    Ok(())
}
