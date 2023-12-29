#[derive(Eq, PartialEq, Hash)]
pub enum LinkType {
    Github,
}

pub enum TemplateType {
    Title {
        title: String,
    },
    Paragraph {
        content: String,
    },
    Links {
        links: std::collections::HashMap<LinkType, String>,
    },
    Navbar {
        paths: Vec<std::path::PathBuf>,
    },
    NRCMSInfo {
        text: &'static str,
    },
    Image {
        url: String,
        copy_asset: bool,
        size: Option<u32>,
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
    pub fn get_links(&self) -> Option<&std::collections::HashMap<LinkType, String>> {
        if let TemplateType::Links { links } = self {
            return Some(links);
        }
        return None;
    }
    pub fn get_navbar(&self) -> Option<&Vec<std::path::PathBuf>> {
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
}

pub struct CMSSite {
    pub original_content: String,
    pub templates: Vec<TemplateType>,
}
