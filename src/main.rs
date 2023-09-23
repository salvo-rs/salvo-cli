use anyhow::Result;
use clap::Parser;
use std::{net::IpAddr};
// use handlebars::Handlebars;
// use serde_json::json;
use dialoguer::{console::Style, theme::ColorfulTheme, Confirm, Input, Select};
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

#[derive(Parser, Debug)]
pub struct Project {
    pub project_name: String,
}
fn main() -> Result<()> {
    utils::print_logo();
    let config=init_config()?;
    dbg!(config);
    // match init_config() {
    //     Ok(None) => println!("Aborted."),
    //     Ok(Some(config)) => println!("{:#?}", config),
    //     Err(err) => println!("error: {}", err),
    // }


    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Optionally pick your flavor")
    //     .default(0)
    //     .items(&selections[..])
    //     .interact_opt()
    //     .unwrap();

    // if let Some(selection) = selection {
    //     println!("Enjoy your {}!", selections[selection]);
    // } else {
    //     println!("You didn't select anything!");
    // }

    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Optionally pick your flavor, hint it might be on the second page")
    //     .default(0)
    //     .max_length(2)
    //     .items(&selections[..])
    //     .interact()
    //     .unwrap();

    // println!("Enjoy your {}!", selections[selection]);

    // let mut reg = Handlebars::new();
    // // render without register
    // println!(
    //     "{}",
    //     reg.render_template("Hello {{name}}", &json!({"name": "foo"}))?
    // );
    // // register template using given name
    // reg.register_template_string("tpl_1", "Good afternoon, {{name}}")?;
    // println!("{}", reg.render("tpl_1", &json!({"name": "foo"}))?);
    let opts: Opts = Opts::parse();
    //dbg!(opts);
    match opts.subcmd {
        SubCommand::New(project) => {
            match utils::create_project(project)  {
                Ok(_) => (),
                Err(e) => utils::error(e.to_string()),
            };
        }
    }
    Ok(())
}
#[derive(Debug)]
struct Config {
    template_type: TemplateType,
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
        .with_prompt(" Welcome to use salvo cli, please choose a template type\n   space bar to confirm")
        .default(0)
        .items(&selections[..])
        .interact()?;
    println!("Enjoy your {}!", &selections[selection]);
    let template_type= match selection {
        0 => TemplateType::SalvoWebApi,
        1 => TemplateType::SalvoWebSite,
        _ => anyhow::bail!("Invalid selection"),
    };
    Ok(Some(Config {
        template_type,
    }))
}
#[derive(Debug)]
pub enum TemplateType{
    SalvoWebSite,
    SalvoWebApi,
}