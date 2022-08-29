use url::Url;

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub surname: String,
    pub office: String,
    pub uni_email: Option<String>,
    pub email: String,
    pub phone_number: Option<String>,
    pub site: Option<Url>,
}
