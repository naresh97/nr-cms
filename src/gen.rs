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

pub fn generate_website(cmsfile: &CMSFile) -> String {
    let title = get_title(cmsfile);
    let site = format!(r#"
    <html>
    <head>
    <title>{title}</title>
    </head>
    <body>
    <h1>{title}</h1>
    </body>
    </html>
    "#);
    return site;
}