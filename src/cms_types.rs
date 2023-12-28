#[derive(Eq, PartialEq, Hash)]
pub enum LinkType {
    Github
}

pub enum TemplateType {
    Title {
        title: String
    },
    Paragraph {
        content: String
    },
    Links {
        links: std::collections::HashMap<LinkType, String>
    },
    Navbar {
        paths: Vec<std::path::PathBuf>
    },
    NRCMSInfo {
        text: &'static str
    },
}

pub struct CMSFile {
    pub original_content: String,
    pub templates: Vec<TemplateType>,
}