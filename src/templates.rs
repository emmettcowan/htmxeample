use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate<'a> {
    pub name: &'a str,
    pub items: Vec<&'a str>,
}

#[derive(Template)]
#[template(path = "profile.html")]
pub struct ProfileTemplate {}
