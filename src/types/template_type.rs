use std::collections::HashMap;

use super::link_type::LinkType;

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
}

impl TemplateType {
    pub fn get_title(&self) -> Option<&String> {
        if let TemplateType::Title { title } = self {
            return Some(title);
        }
        return None;
    }
    pub fn get_paragraph(&self) -> Option<&String> {
        if let TemplateType::Paragraph { content } = self {
            return Some(content);
        }
        return None;
    }
    pub fn get_links(&self) -> Option<&HashMap<LinkType, String>> {
        if let TemplateType::Links { links } = self {
            return Some(links);
        }
        return None;
    }
    pub fn get_navbar(&self) -> Option<&Vec<String>> {
        if let TemplateType::Navbar { paths } = self {
            return Some(paths);
        }
        return None;
    }
    pub fn get_nr_cms_info(&self) -> Option<&str> {
        if let TemplateType::NRCMSInfo { text } = self {
            return Some(text);
        }
        return None;
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
        return None;
    }
    pub fn get_name(&self) -> Option<&str> {
        if let TemplateType::Name { name } = self {
            return Some(name);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_title() {
        let a = TemplateType::Title {
            title: String::from("test"),
        };
        assert_eq!(a.get_title().unwrap(), "test");
        let b = TemplateType::Name {
            name: String::from("abc"),
        };
        assert!(b.get_title().is_none());
    }

    #[test]
    fn test_get_paragraph() {
        let a = TemplateType::Paragraph {
            content: String::from("test"),
        };
        assert_eq!(a.get_paragraph().unwrap(), "test");
        let b = TemplateType::Name {
            name: String::from("abc"),
        };
        assert!(b.get_paragraph().is_none());
    }

    #[test]
    fn test_get_links() {
        let a = TemplateType::Links {
            links: HashMap::from([(LinkType::Github, String::from("test"))]),
        };
        assert_eq!(
            a.get_links().unwrap().get(&LinkType::Github).unwrap(),
            "test"
        );
        let b = TemplateType::Name {
            name: String::from("abc"),
        };
        assert!(b.get_links().is_none());
    }

    #[test]
    fn test_get_nr_cms_info() {
        let a = TemplateType::NRCMSInfo { text: "test" };
        assert_eq!(a.get_nr_cms_info().unwrap(), "test");
        let b = TemplateType::Name {
            name: String::from("abc"),
        };
        assert!(b.get_nr_cms_info().is_none());
    }

    #[test]
    fn test_get_image() {
        let a = TemplateType::Image {
            url: String::from("test"),
            copy_asset: false,
            size: Some(10),
        };
        assert_eq!(a.get_image().unwrap().0, "test");
        let b = TemplateType::Name {
            name: String::from("abc"),
        };
        assert!(b.get_image().is_none());
    }

    #[test]
    fn test_get_name() {
        let a = TemplateType::Name {
            name: String::from("test"),
        };
        assert_eq!(a.get_name().unwrap(), "test");
        let b = TemplateType::Title {
            title: String::from("test"),
        };
        assert!(b.get_name().is_none());
    }

    #[test]
    fn test_get_navbar() {
        let a = TemplateType::Navbar {
            paths: Vec::from([String::from("test")]),
        };
        assert_eq!(a.get_navbar().unwrap().get(0).unwrap(), "test");
        let b = TemplateType::Title {
            title: String::from("test"),
        };
        assert!(b.get_navbar().is_none());
    }
}
