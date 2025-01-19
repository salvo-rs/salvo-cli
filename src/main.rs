#![recursion_limit = "256"]

use anyhow::Result;
use clap::Parser;
mod tests;
mod utils;
use i18n::set_locale;
mod i18n;
mod git;
mod templates;
mod project;
// mod updater;
mod printer;
mod namer;

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
    printer::print_logo();
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::New(new_cmd) => {
            set_locale(&new_cmd.lang);
            // updater::check_for_updates().await;
            match project::create(&new_cmd) {
                Ok(_) => (),
                Err(e) => printer::error(e.to_string()),
            };
        }
    }
    Ok(())
}


