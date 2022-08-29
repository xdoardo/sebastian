#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArielSitemap {
    pub login_url: String,
    pub search_url: String,
    pub home_page_url: String,
}

impl Default for ArielSitemap {
    fn default() -> ArielSitemap {
        ArielSitemap {
            login_url: "https://elearning.unimi.it/authentication/skin/portaleariel/login.aspx?url=https://ariel.unimi.it/".to_string(), 
            search_url:  "https://ariel.unimi.it/offerta/search/quick".to_string(),
            home_page_url: "https://ariel.unimi.it/".to_string()
        }
    }
}
