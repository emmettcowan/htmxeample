use askama::Template;

#[derive(Template)]
#[template(path = "Hello.html")]
pub struct HelloTemplate<'a> {
    pub name: &'a str,
    pub items: Vec<&'a str>,
}
