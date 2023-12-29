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

pub struct CMSFile {
    pub original_content: String,
    pub templates: Vec<TemplateType>,
}
