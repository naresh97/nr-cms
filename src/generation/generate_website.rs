use crate::{
    assets, parsing,
    types::{cms_site::CMSSite, generation_dirs::GenerationDirs},
};

use super::{page_generator::*, template_generators::*};

pub fn generate_website(generation_dirs: &impl GenerationDirs) {
    let index_file = parsing::parse_file(generation_dirs);
    match index_file {
        Ok(index_file) => {
            let html = generate_html(&index_file, generation_dirs);
            write_file(generation_dirs.in_gen("index.html"), &html).unwrap_or_else(|e| {
                log::error!("Could not write HTML to file: {}", e.to_string());
            });
        }
        Err(e) => {
            log::error!("Could not load CMS site: {}", e.to_string());
        }
    }
}

fn write_file(file_path: std::path::PathBuf, html: &str) -> Result<(), std::io::Error> {
    log::info!("Generating website");
    let parent = file_path.parent().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not get parent",
    ))?;
    std::fs::create_dir_all(parent)?;
    std::fs::write(file_path, html)?;
    Ok(())
}

fn generate_html(cms_site: &CMSSite, generation_dirs: &impl GenerationDirs) -> String {
    let templates = &cms_site.templates;
    let pages = &cms_site.pages;
    let title = gen_title(templates);
    let navbar = gen_navbar(templates);
    let nr_cms_info = gen_nr_cms_info(templates);
    let style = assets::styles::SITE_STYLE;
    let script = assets::scripts::PAGE_LOGIC;
    let pages = gen_pages(pages, generation_dirs);
    let site = format!(
        r#"
    <html>
    <head>
    {script}
    {style}
    <title>{title}</title>
    </head>
    <body>
    <h1>{title}</h1>
    {navbar}
    {pages}
    {nr_cms_info}
    </body>
    </html>
    "#
    );
    site
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::types::{
        cms_page::CMSPage, cms_site::CMSSite, generation_dirs::TempGenerationDirs,
        template_type::TemplateType,
    };

    use super::*;

    #[test]
    fn test_generate_html() {
        let cms_site = CMSSite {
            original_content: Default::default(),
            templates: Vec::from([TemplateType::Title {
                title: "TestSite".to_string(),
            }]),
            pages: HashMap::<String, CMSPage>::new(),
        };
        let html = generate_html(&cms_site, &TempGenerationDirs::default());
        assert!(html.contains("TestSite"));
    }

    #[test]
    fn test_write_to_file() {
        let tmpfile = tempfile::tempdir().unwrap();
        let path = tmpfile.path().join("test.file");
        write_file(path.to_path_buf(), "content").unwrap();
        assert!(path.exists());
    }
}
