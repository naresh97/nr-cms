use std::collections::HashMap;

use chrono::{DateTime, Utc};

use super::{cms_blog::CMSBlog, link_type::LinkType};

#[derive(Clone)]
pub enum TemplateType {
    Title {
        title: String,
    },
    Paragraph {
        content: String,
    },
    Links {
        links: HashMap<LinkType, String>,
    },
    Navbar {
        paths: Vec<String>,
    },
    NRCMSInfo {
        text: &'static str,
    },
    Image {
        url: String,
        copy_asset: bool,
        size: Option<u32>,
    },
    Name {
        name: String,
    },
    Date {
        date: DateTime<Utc>,
    },
    Blog(CMSBlog),
    Code {
        code: String,
    },
}

impl TemplateType {
    pub fn get_title(&self) -> Option<&String> {
        if let TemplateType::Title { title } = self {
            return Some(title);
        }
        None
    }
    pub fn get_paragraph(&self) -> Option<&String> {
        if let TemplateType::Paragraph { content } = self {
            return Some(content);
        }
        None
    }
    pub fn get_links(&self) -> Option<&HashMap<LinkType, String>> {
        if let TemplateType::Links { links } = self {
            return Some(links);
        }
        None
    }
    pub fn get_navbar(&self) -> Option<&Vec<String>> {
        if let TemplateType::Navbar { paths } = self {
            return Some(paths);
        }
        None
    }
    pub fn get_nr_cms_info(&self) -> Option<&str> {
        if let TemplateType::NRCMSInfo { text } = self {
            return Some(text);
        }
        None
    }
    pub fn get_image(&self) -> Option<(&String, &bool, &Option<u32>)> {
        if let TemplateType::Image {
            url,
            copy_asset,
            size,
        } = self
        {
            return Some((url, copy_asset, size));
        }
        None
    }
    pub fn get_name(&self) -> Option<&str> {
        if let TemplateType::Name { name } = self {
            return Some(name);
        }
        None
    }
    pub fn get_date(&self) -> Option<&DateTime<Utc>> {
        if let TemplateType::Date { date } = self {
            return Some(date);
        }
        None
    }
    pub fn get_blog(&self) -> Option<&CMSBlog> {
        if let TemplateType::Blog(blog) = self {
            return Some(blog);
        }
        None
    }
    pub fn get_code(&self) -> Option<&String> {
        if let TemplateType::Code { code } = self {
            return Some(code);
        }
        None
    }
}

pub trait TemplateTypeVector {
    fn get_title(&self) -> Option<&String>;
    fn get_navbar(&self) -> Option<&Vec<String>>;
    fn get_paragraphs(&self) -> Vec<&String>;
    fn get_links(&self) -> Option<&HashMap<LinkType, String>>;
    fn get_nr_cms_info(&self) -> Option<&str>;
    fn get_image(&self) -> Option<(&String, &bool, &Option<u32>)>;
    fn get_date(&self) -> Option<&DateTime<Utc>>;
    fn get_blog(&self) -> Option<&CMSBlog>;
    fn get_code(&self) -> Option<&String>;
}

impl TemplateTypeVector for Vec<TemplateType> {
    fn get_title(&self) -> Option<&String> {
        self.iter().find_map(|x| x.get_title())
    }

    fn get_navbar(&self) -> Option<&Vec<String>> {
        self.iter().find_map(|x| x.get_navbar())
    }

    fn get_paragraphs(&self) -> Vec<&String> {
        self.iter().filter_map(|x| x.get_paragraph()).collect()
    }

    fn get_links(&self) -> Option<&HashMap<LinkType, String>> {
        self.iter().find_map(|x| x.get_links())
    }

    fn get_nr_cms_info(&self) -> Option<&str> {
        self.iter().find_map(|x| x.get_nr_cms_info())
    }

    fn get_image(&self) -> Option<(&String, &bool, &Option<u32>)> {
        self.iter().find_map(|x| x.get_image())
    }

    fn get_date(&self) -> Option<&DateTime<Utc>> {
        self.iter().find_map(|x| x.get_date())
    }

    fn get_blog(&self) -> Option<&CMSBlog> {
        self.iter().find_map(|x| x.get_blog())
    }

    fn get_code(&self) -> Option<&String> {
        self.iter().find_map(|x| x.get_code())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_title() {
        let a = TemplateType::Title {
            title: "test".to_string(),
        };
        assert_eq!(a.get_title().unwrap(), "test");
        let b = TemplateType::Name {
            name: "abc".to_string(),
        };
        assert!(b.get_title().is_none());
    }

    #[test]
    fn test_get_paragraph() {
        let a = TemplateType::Paragraph {
            content: "test".to_string(),
        };
        assert_eq!(a.get_paragraph().unwrap(), "test");
        let b = TemplateType::Name {
            name: "abc".to_string(),
        };
        assert!(b.get_paragraph().is_none());
    }

    #[test]
    fn test_get_links() {
        let a = TemplateType::Links {
            links: HashMap::from([(LinkType::Github, "test".to_string())]),
        };
        assert_eq!(
            a.get_links().unwrap().get(&LinkType::Github).unwrap(),
            "test"
        );
        let b = TemplateType::Name {
            name: "abc".to_string(),
        };
        assert!(b.get_links().is_none());
    }

    #[test]
    fn test_get_nr_cms_info() {
        let a = TemplateType::NRCMSInfo { text: "test" };
        assert_eq!(a.get_nr_cms_info().unwrap(), "test");
        let b = TemplateType::Name {
            name: "abc".to_string(),
        };
        assert!(b.get_nr_cms_info().is_none());
    }

    #[test]
    fn test_get_image() {
        let a = TemplateType::Image {
            url: "test".to_string(),
            copy_asset: false,
            size: Some(10),
        };
        assert_eq!(a.get_image().unwrap().0, "test");
        let b = TemplateType::Name {
            name: "abc".to_string(),
        };
        assert!(b.get_image().is_none());
    }

    #[test]
    fn test_get_name() {
        let a = TemplateType::Name {
            name: "test".to_string(),
        };
        assert_eq!(a.get_name().unwrap(), "test");
        let b = TemplateType::Title {
            title: "test".to_string(),
        };
        assert!(b.get_name().is_none());
    }

    #[test]
    fn test_get_navbar() {
        let a = TemplateType::Navbar {
            paths: Vec::from(["test".to_string()]),
        };
        assert_eq!(a.get_navbar().unwrap().get(0).unwrap(), "test");
        let b = TemplateType::Title {
            title: "test".to_string(),
        };
        assert!(b.get_navbar().is_none());
    }
}
