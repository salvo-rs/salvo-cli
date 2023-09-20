use std::{fs, io::Write, path::Path};

use anyhow::Result;
use clap::Parser;
// use handlebars::Handlebars;
// use serde_json::json;

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
struct Project {
    project_name: String,
}
fn main() -> Result<()> {
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
            let project_path = Path::new(project.project_name.as_str());
            // 创建 main.rs 文件
            dbg!(&project_path);
            // 创建项目目录
            if !project_path.exists() {
                fs::create_dir_all(&project_path)?;
            }
            // 创建 src 目录
            let src_path = project_path.join("src");
            if !src_path.exists() {
                dbg!(2.1,&src_path);
                fs::create_dir_all(&src_path)?;
            }
            dbg!(3);
            // 创建 main.rs 文件
            let main_file_path = src_path.join("main.rs");
            if !main_file_path.exists() {
                let mut main_file = fs::File::create(main_file_path)?;
                main_file.write_all(b"use salvo::prelude::*;
#[handler]
async fn hello() -> &'static str {
    \"Hello World\"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello);
    let acceptor = TcpListener::new(\"127.0.0.1:5800\").bind().await;
    Server::new(acceptor).serve(router).await;
}
                ")?;
            }
            
            // 创建 Cargo.toml 文件
            let cargo_file_path = project_path.join("Cargo.toml");
            if !cargo_file_path.exists() {
                let mut cargo_file = fs::File::create(cargo_file_path)?;
                cargo_file.write_all(b"\
[package]
name = \"code\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
salvo = \"0.55\"
tokio = { version = \"1\", features = [\"macros\"] }
tracing = \"0.1\"
tracing-subscriber = \"0.3\"
                ")?;
            }
         }
    }
    Ok(())
}
