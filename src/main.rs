#![recursion_limit = "256"]

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
    New(NewCmd),
}
#[derive(Parser, Debug, Clone)]
pub struct NewCmd {
    pub project_name: String,
    #[clap(short, long)]
    lang: Option<String>,
}
#[tokio::main]
async fn main() -> Result<()> {
    utils::print_logo();
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::New(new_cmd) => {
            set_locale(&new_cmd.lang);
            check_for_updates().await;
            match utils::create_project(new_cmd.project_name) {
                Ok(_) => (),
                Err(e) => utils::error(e.to_string()),
            };
        }
    }
    Ok(())
}
