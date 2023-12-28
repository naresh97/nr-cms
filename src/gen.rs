use crate::cms_types::{CMSFile, TemplateType};

fn get_title(cms_file: &CMSFile) -> &str {
    let title = cms_file.templates.iter().find(|x| {
        if let TemplateType::Title { title: _title } = x {
            return true;
        }
        return false;
    });
    match title {
        Some(TemplateType::Title { title: m_title }) => m_title,
        _ => ""
    }
}

fn get_navbar(cms_file: &CMSFile) -> String {
    let navbar = cms_file.templates.iter().find(|x| {
        match x {
            TemplateType::Navbar { paths } => true,
            _ => false
        }
    });
    match navbar {
        Some(TemplateType::Navbar { paths }) => {
            let mut links = String::new();
            let paths: Vec<String> = paths.iter().filter_map(|x| {
                let path_str = x.to_str();
                match path_str {
                    Some(path_str) => Some(format!("<a href=\"{path_str}\">{path_str}</a>")),
                    _ => None
                }
            }).collect();
            let paths = paths.join(" | ");
            paths
        }
        _ => String::new()
    }
}

pub fn generate_website(cmsfile: &CMSFile) -> String {
    let title = get_title(cmsfile);
    let navbar = get_navbar(cmsfile);
    let site = format!(r#"
    <html>
    <head>
    <title>{title}</title>
    </head>
    <body>
    <h1>{title}</h1>
    {navbar}
    </body>
    </html>
    "#);
    return site;
}