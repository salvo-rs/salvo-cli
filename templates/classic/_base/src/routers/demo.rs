use rinja::Template;
use salvo::prelude::*;

use crate::AppResult;

#[handler]
pub async fn hello(req: &mut Request) -> AppResult<Text<String>> {
    #[derive(Template)]
    #[template(path = "hello.html")]
    struct HelloTemplate<'a> {
        name: &'a str,
    }
    let hello_tmpl = HelloTemplate {
        name: req.query::<&str>("name").unwrap_or("World"),
    };
    Ok(Text::Html(hello_tmpl.render().unwrap()))
}