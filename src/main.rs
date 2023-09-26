use anyhow::Result;
use clap::Parser;
use dialoguer::{console::Style, theme::ColorfulTheme, Select};

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
// get 子命令

#[derive(Parser, Debug, Clone)]
pub struct Project {
    pub project_name: String,
}
fn main() -> Result<()> {
    utils::print_logo();
    let config = init_config()?;
    match config {
        Some(config) => {
            let opts: Opts = Opts::parse();
            //dbg!(opts);
            match opts.subcmd {
                SubCommand::New(project) => {
                    match utils::create_project(project, config) {
                        Ok(_) => (),
                        Err(e) => utils::error(e.to_string()),
                    };
                }
            }
        }
        None =>     anyhow::bail!("failed to create project")        ,
    }
    Ok(())
}
#[derive(Debug)]
pub struct Config {
    pub template_type: TemplateType,
}

fn init_config() -> Result<Option<Config>> {
    let theme = ColorfulTheme {
        defaults_style: Style::new().blue(),
        prompt_style: Style::new().green().bold(),
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    let selections = &[
        "salvo_web_api (Default web api template) ",
        "salvo_web_site (Default web site template)",
        // "custom",
    ];
    let selection = Select::with_theme(&theme)
        .with_prompt(
            " Welcome to use salvo cli, please choose a template type\n   space bar to confirm",
        )
        .default(0)
        .items(&selections[..])
        .interact()?;
    println!("Enjoy your {}!", &selections[selection]);
    let template_type = match selection {
        0 => TemplateType::SalvoWebApi,
        1 => TemplateType::SalvoWebSite,
        _ => anyhow::bail!("Invalid selection"),
    };
    Ok(Some(Config { template_type }))
}
#[derive(Debug,PartialEq)]
pub enum TemplateType {
    SalvoWebSite,
    SalvoWebApi,
}
