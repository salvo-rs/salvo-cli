use anyhow::Result;
use clap::Parser;

mod utils;

#[derive(Parser, Debug)]
#[clap(version = "0.0.1", author = "Fankai liu <liufankai137@outlook.com>")]
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
}
fn main() -> Result<()> {
    utils::print_logo();
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::New(project) => {
            match utils::create_project(project) {
                Ok(_) => (),
                Err(e) => utils::error(e.to_string()),
            };
        }
    }
    Ok(())
}
