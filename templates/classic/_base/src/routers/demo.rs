
use askama::Template;
use salvo::prelude::*;
use crate::AppResult;

#[handle]
pub async fn hello(req: &mut Request, res: &mut Response)->AppResult<Text>{
    #[derive(Template)]
    #[template(path = "hello.html")]
    struct HelloTemplate<'a> {
        name: &'a str,
    }
    let hello_tmpl = HelloTemplate {
        name: req.param::<&str>("name").unwrap_or("World"),
    };
    Ok(Text::Html(hello_tmpl.render().unwrap()))
}

mod tests {
    use salvo::test::{ResponseExt, TestClient};
    use salvo::Service;
    use crate::config::CFG;

    #[tokio::test]
    async fn test_hello_world() {
        let service = Service::new(crate::routers::router());

        let content = TestClient::get(format!(
            "http://{}",
            &config.listen_addr.replace("0.0.0.0", "127.0.0.1")
        ))
        .send(&service)
        .await
        .take_string()
        .await
        .unwrap();
        assert_eq!(content, "Hello World from salvo");
    }
}
