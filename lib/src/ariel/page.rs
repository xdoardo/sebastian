use std::rc::Rc;

use soup::{NodeExt, QueryBuilderExt, Soup};
use url::Url;

pub struct ArielLoginPage {}
impl ArielLoginPage {
    pub fn is_logged_in(raw: String) -> anyhow::Result<()> {
        let soup = Soup::new(&raw);
        if let Some(s) = soup
            .tag("span")
            .attr("id", "cvLogin")
            .attr("class", "text-danger")
            .find()
        {
            anyhow::bail!("'{}'", s.text())
        }
        Ok(())
    }
}

pub struct ArielSearchPage {}
impl ArielSearchPage {
    pub fn title_pages(raw: String) -> Vec<ArielTitlePage> {
        log::debug!("raw: {}", raw);
        let mut res = vec![];
        let soup = Soup::new(&raw);
        if let Some(sites) = soup.tag("div").attr("id", "sitiariel").find() {
            for site in sites.tag("div").attr("class", "ariel-project").find_all() {
                if let Some(href) = site.tag("a").attr("class", "ariel").find() {
                    let url = href.get("href").unwrap();
                    let title = href.text();
                    if let Some(teachers) = site.tag("ul").attr("class", "list-user").find() {
                        let mut tas = vec![];
                        for teacher in teachers
                            .tag("a")
                            .attr("href", regex::Regex::new(r".*teacher.*").unwrap())
                            .find_all()
                        {
                            tas.push(teacher.text());
                        }
                        if let Some(_) = site.tag("span").attr("class", "bg-tag-danger").find() {
                            res.push(ArielTitlePage {
                                title,
                                url: url.parse::<url::Url>().unwrap(),
                                holders: tas,
                                can_access: false,
                            })
                        } else if let Some(_) =
                            site.tag("span").attr("class", "bg-tag-success").find()
                        {
                            res.push(ArielTitlePage {
                                title,
                                url: url.parse::<url::Url>().unwrap(),
                                holders: tas,
                                can_access: true,
                            })
                        }
                    }
                }
            }
        }
        res
    }
}

#[derive(Debug, Clone)]
pub struct ArielTitlePage {
    pub title: String,
    pub url: Url,
    pub holders: Vec<String>,
    pub can_access: bool,
}

impl std::fmt::Display for ArielTitlePage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)?;
        let mut holders = String::from("[");
        for (i, holder) in self.holders.clone().into_iter().enumerate() {
            if i < self.holders.len() - 1 {
                holders.push_str(format!("{}, ", holder).as_str());
            } else {
                holders.push_str(holder.as_str());
            }
        }
        holders.push_str("]");
        write!(f, " {}", holders)?;
        if self.can_access {
            write!(f, " accessible")
        } else {
            write!(f, " non accessible")
        }
    }
}

#[derive(Debug)]
pub struct ArielPage {
    pub soup: Soup,
    pub url: String,
}

impl ArielPage {
    pub fn from_raw(raw: String, url: String) -> ArielPage {
        ArielPage {
            soup: Soup::new(&raw),
            url,
        }
    }

    pub fn get_title(&self) -> String {
        todo!()
    }

    pub fn get_children(&self) -> Vec<String> {
        if let Some(meta) = self.soup.tag("meta").find() {
            if let Some(content) = meta.get("content") {
                if content.contains("0; URL=v5") {
                    let url = Url::parse(&self.url).unwrap();
                    let url = url.join("v5").unwrap();
                    return vec![url.to_string()];
                }
            }
        }
        if let Some(t) = self
            .soup
            .tag("a")
            .attr("href", regex::Regex::new("toolName=cont.*").unwrap())
            .find()
        {
            if let Some(li) = self.soup.tag("li").attr("class", "active").find() {
                if let None = li
                    .tag("a")
                    .attr("href", regex::Regex::new("toolName=cont.*").unwrap())
                    .find()
                {
                    let mut url = self.url.clone().parse::<Url>().unwrap();
                    url = url.join(&t.get("href").unwrap()).unwrap();
                    return vec![url.to_string()];
                } else {
                    if let Some(tbody) =
                        self.soup.tag("tbody").attr("class", "arielRoomList").find()
                    {
                        let hrefs = tbody
                            .tag("a")
                            .attr("href", regex::Regex::new(r"ThreadList.*").unwrap())
                            .find_all();
                        let mut res = vec![];
                        for href in hrefs {
                            if let Some(value) = href.get("href") {
                                let mut url = self.url.clone().parse::<Url>().unwrap();
                                url = url.join(&value).unwrap();
                                res.push(url.to_string());
                            }
                        }
                        return res;
                    } else {
                        if let Some(tbody) = self
                            .soup
                            .tag("tbody")
                            .attr("class", "arielThreadList")
                            .find()
                        {
                            let hrefs = tbody
                                .tag("a")
                                .attr("href", regex::Regex::new(r"ThreadList.*").unwrap())
                                .find_all();
                            let mut res = vec![];
                            for href in hrefs {
                                if let Some(value) = href.get("href") {
                                    let mut url = self.url.clone().parse::<Url>().unwrap();
                                    url = url.join(&value).unwrap();
                                    res.push(url.to_string());
                                }
                            }
                            return res;
                        } else {
                            return vec![];
                        }
                    }
                }
            }
        }

        let hrefs = self
            .soup
            .tag("a")
            .attr(
                "href",
                regex::Regex::new(r".*//.*\.ctu\.unimi\.it.*").unwrap(),
            )
            .find_all();
        let mut res = vec![];
        for href in hrefs {
            if let Some(value) = href.get("href") {
                if value.contains("www.ctu") {
                    continue;
                }
                res.push(value);
            }
        }
        res
    }

    pub fn get_data(&self) -> Vec<ArielPageData> {
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct ArielPageData {
    pub name: String,
    pub url: Url,
    pub origin: Rc<ArielPage>,
}

impl std::fmt::Display for ArielPageData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
