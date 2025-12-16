use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "index.html", escape = "none")]
pub struct Index<'a> {
    pub name: &'a str,
    pub items: Vec<&'a str>,
}

#[derive(Template, WebTemplate)]
#[template(path = "profile.html")]
pub struct ProfileTemplate {}
