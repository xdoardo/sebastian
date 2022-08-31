use std::rc::Rc;

use url::Url;

pub struct ArielLoginPage {}
impl ArielLoginPage {
    pub fn is_logged_in(raw: String) -> anyhow::Result<()> {
        let mut options = tl::ParserOptions::new();
        options = options.track_ids();
        options = options.track_classes();
        let soup = tl::parse(&raw, options).unwrap();
        let parser = soup.parser();

        if let Some(cv_login) = soup.get_element_by_id("cvLogin") {
            if let Some(_) = cv_login
                .get(parser)
                .unwrap()
                .find_node(parser, &mut |child| {
                    if let tl::Node::Tag(tag) = child {
                        if let Some(Some(text_danger)) = tag.attributes().get("class") {
                            return text_danger == "text-danger";
                        }
                    }
                    return false;
                })
            {
                anyhow::bail!("")
            }
        }
        Ok(())
    }
}

pub struct ArielSearchPage {}
impl ArielSearchPage {
    pub fn title_pages(raw: String) -> Vec<ArielTitlePage> {
        log::debug!("raw: {}", raw);
        let mut res = vec![];

        let mut options = tl::ParserOptions::new();
        options = options.track_ids();
        options = options.track_classes();
        let soup = tl::parse(&raw, options).unwrap();
        let parser = soup.parser();

        for handle in soup.get_elements_by_class_name("ariel-project") {
            let mut url = String::new();
            let mut title = String::new();
            let teacher_url_regex = regex::Regex::new(r".*teacher.*").unwrap();
            let mut holders = vec![];
            let mut can_access = false;
            let node = handle.get(parser).unwrap();

            if let Some(children) = node.children() {
                for child in children.all(parser) {
                    if let tl::Node::Tag(child) = child {
                        let mut child_class = String::new();
                        let mut child_href = String::new();

                        if child.attributes().contains("class") {
                            child_class = child
                                .attributes()
                                .get("class")
                                .unwrap()
                                .unwrap()
                                .as_utf8_str()
                                .to_string();
                        }
                        if child.attributes().contains("href") {
                            child_href = child
                                .attributes()
                                .get("href")
                                .unwrap()
                                .unwrap()
                                .as_utf8_str()
                                .to_string();
                        }
                        if child_class == "ariel" {
                            title = child.inner_text(parser).to_string();
                            url = child_href
                        } else if teacher_url_regex.is_match(child_href.as_str()) {
                            holders.push(child_href);
                            println!("teachers inner text: {}", child.inner_text(parser))
                        } else if child_class == "bg-tag-success" {
                            can_access = true
                        }
                    }
                }
            }
            res.push(ArielTitlePage {
                url: url.parse::<url::Url>().unwrap(),
                title,
                holders,
                can_access,
            });
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
enum ArielPageKind {
    HomePage,
    SiteHomePage,
    SiteAmbient,
    Unknown,
}

#[derive(Debug)]
pub struct ArielPage {
    pub url: String,
    pub soup: tl::VDomGuard,
    kind: ArielPageKind,
}

impl ArielPage {
    pub fn from_raw(raw: String, url: String) -> anyhow::Result<ArielPage> {
        let mut options = tl::ParserOptions::new();
        options = options.track_ids();
        options = options.track_classes();
        let soup = unsafe { tl::parse_owned(raw, options) }?;
        let mut kind = ArielPageKind::Unknown;
        let parser = soup.get_ref().parser();

        if let Some(navbar) = soup.get_ref().get_element_by_id("bs-navbar") {
            if let Some(navbar) = navbar.get(parser) {
                if let Some(c) = navbar.find_node(parser, &mut |child| {
                    if let tl::Node::Tag(h) = child {
                        if let Some(Some(class)) = h.attributes().get("class") {
                            if class == "active" {
                                return true;
                            }
                        }
                    }
                    false
                }) {
                    if let Some(c) = c.get(parser) {
                        if c.inner_text(parser).contains("Home") {
                            kind = ArielPageKind::HomePage;
                        }
                    }
                }
            }
        } else if let Some(_) = soup.get_ref().get_element_by_id("ctl24_lblProjectTitle") {
            for ul in soup.get_ref().get_elements_by_class_name("navbar-nav") {
                if let Some(ul) = ul.get(parser) {
                    if let Some(children) = ul.children() {
                        for child in children.all(parser) {
                            if let tl::Node::Tag(child) = child {
                                if let Some(Some(class)) = child.attributes().get("class") {
                                    if class == "active" {
                                        let inner = child.inner_text(parser);
                                        if inner.to_lowercase().contains("home") {
                                            kind = ArielPageKind::SiteHomePage;
                                        } else if inner.to_lowercase().contains("conten") {
                                            kind = ArielPageKind::SiteAmbient;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(ArielPage { url, soup, kind })
    }

    pub fn get_title(&self) -> String {
        let str = match self.kind {
            ArielPageKind::HomePage => String::from("Ariel"),
            ArielPageKind::SiteHomePage => {
                let parser = self.soup.get_ref().parser();
                let mut ret = String::new();
                if let Some(title) = self
                    .soup
                    .get_ref()
                    .get_element_by_id("ctl24_lblProjectTitle")
                {
                    if let Some(title) = title.get(parser) {
                        let title = title.inner_text(parser);
                        ret = title.to_string().trim().to_string();
                    }
                }
                ret
            }
            ArielPageKind::SiteAmbient => {
                let parser = self.soup.get_ref().parser();
                let mut site_title = String::new();
                if let Some(title) = self
                    .soup
                    .get_ref()
                    .get_element_by_id("ctl24_lblProjectTitle")
                {
                    if let Some(title) = title.get(parser) {
                        let title = title.inner_text(parser);
                        site_title = title.trim().to_string();
                    }
                }

                for h1 in self.soup.get_ref().get_elements_by_class_name("arielTitle") {
                    if let Some(h1) = h1.get(parser) {
                        if let tl::Node::Tag(h1) = h1 {
                            if h1.name() == "h1" {
                                let page_title = h1.inner_text(parser).trim().to_string();
                                if !site_title.is_empty() {
                                    site_title.push_str(" - ")
                                }
                                site_title.push_str(page_title.to_string().trim());
                            }
                        }
                    }
                }
                site_title
            }

            ArielPageKind::Unknown => format!("Unknown ({})", self.url),
        };
        str
    }

    pub fn get_children(&self) -> Vec<String> {
        match self.kind {
            ArielPageKind::HomePage => self.children_ariel_home(),
            ArielPageKind::SiteHomePage => self.children_site_home_page(),
            ArielPageKind::SiteAmbient => self.children_ambient(),
            ArielPageKind::Unknown => vec![],
        }
    }

    pub fn get_data(&self) -> Vec<ArielPageData> {
        // @TODO!
        vec![]
    }

    fn children_ariel_home(&self) -> Vec<String> {
        let parser = self.soup.get_ref().parser();
        let mut res = vec![];
        for ul in self
            .soup
            .get_ref()
            .get_elements_by_class_name("list-unstyled")
        {
            if let Some(ul) = ul.get(parser) {
                if let Some(children) = ul.children() {
                    for child in children.all(parser) {
                        if let tl::Node::Tag(child) = child {
                            if let Some(Some(href)) = child.attributes().get("href") {
                                let href = href.as_utf8_str().to_string();
                                if href.contains("ariel.ctu.unimi.it") {
                                    res.push(href);
                                }
                            }
                        }
                    }
                }
            }
        }
        res
    }

    fn children_site_home_page(&self) -> Vec<String> {
        let parser = self.soup.get_ref().parser();
        let mut res = vec![];
        for ul in self.soup.get_ref().get_elements_by_class_name("navbar-nav") {
            if let Some(ul) = ul.get(parser) {
                if let Some(children) = ul.children() {
                    for child in children.all(parser) {
                        if let tl::Node::Tag(child) = child {
                            if child.name() == "li" {
                                for n in child.children().all(parser) {
                                    if let tl::Node::Tag(n) = n {
                                        if let Some(Some(href)) = n.attributes().get("href") {
                                            let href = href.as_utf8_str().to_string();

                                            if href.contains("toolName=conten") {
                                                return vec![self
                                                    .url
                                                    .parse::<url::Url>()
                                                    .unwrap()
                                                    .join(&href)
                                                    .unwrap()
                                                    .to_string()];
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        res
    }

    fn children_ambient(&self) -> Vec<String> {
        let parser = self.soup.get_ref().parser();
        let mut res = vec![];
        if let Some(rl) = self.soup.get_ref().get_element_by_id("roomList") {
            if let Some(rl) = rl.get(parser) {
                if let Some(children) = rl.children() {
                    for child in children.all(parser) {
                        if let tl::Node::Tag(child) = child {
                            if child.name() == "a" {
                                if let Some(Some(href)) = child.attributes().get("href") {
                                    let href = href.as_utf8_str().to_string().replace("amp;", "");
                                    if href.contains("ThreadList") {
                                        log::info!("found threadlist {}", href);
                                        res.push(
                                            self.url
                                                .parse::<url::Url>()
                                                .unwrap()
                                                .join(&href)
                                                .unwrap()
                                                .to_string(),
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        //        if let Some(tl) = self.soup.get_ref().get_element_by_id("threadList") {
        //            if let Some(tl) = tl.get(parser) {
        //
        //            }
        //        }

        res
    }
}

impl std::fmt::Display for ArielPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_title())
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
