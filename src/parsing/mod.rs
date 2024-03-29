mod get_tags;
mod parse_blog;
mod parse_template_elements;

use std::collections::HashMap;

use crate::types::{
    cms_page::CMSPage, cms_site::CMSSite, generation_dirs::GenerationDirs,
    template_type::TemplateType,
};

use self::{get_tags::get_tags, parse_blog::parse_blog, parse_template_elements::*};

enum ParseElements {
    Template(TemplateType),
    Page(CMSPage),
}

fn parse_page(content: Option<&str>, generation_dirs: &impl GenerationDirs) -> Option<CMSPage> {
    let content = content?;
    let (templates, _pages) = parse_templates(content, generation_dirs);
    Some(CMSPage { templates })
}

fn parse_template(
    template_content: &str,
    generation_dirs: &impl GenerationDirs,
) -> Option<ParseElements> {
    let template_separator = template_content.match_indices('|').next().map(|x| x.0);
    let (template_name, template_content) = match template_separator {
        Some(template_separator) => (
            template_content.get(0..template_separator)?,
            template_content.get(template_separator + 1..),
        ),
        _ => (template_content, None),
    };
    match template_name {
        "Navbar" => parse_navbar(template_content).map(ParseElements::Template),
        "Title" => parse_title(template_content).map(ParseElements::Template),
        "Paragraph" => parse_paragraph(template_content).map(ParseElements::Template),
        "Links" => parse_links(template_content).map(ParseElements::Template),
        "NKR-CMS-INFO" => parse_nkr_cms_info().map(ParseElements::Template),
        "Image" => parse_image(template_content, generation_dirs).map(ParseElements::Template),
        "Name" => parse_name(template_content).map(ParseElements::Template),
        "Page" => parse_page(template_content, generation_dirs).map(ParseElements::Page),
        "Date" => parse_date(template_content).map(ParseElements::Template),
        "Blog" => parse_blog(template_content, generation_dirs).map(ParseElements::Template),
        "Code" => parse_code(template_content).map(ParseElements::Template),
        _ => None,
    }
}

fn parse_templates(
    content: &str,
    generation_dirs: &impl GenerationDirs,
) -> (Vec<TemplateType>, HashMap<String, CMSPage>) {
    let mut result: Vec<TemplateType> = Vec::new();
    let mut pages: HashMap<String, CMSPage> = HashMap::new();

    let tags = get_tags(content);
    if let Some(tags) = tags {
        for template_content in tags {
            let template = parse_template(template_content, generation_dirs);
            if let Some(ParseElements::Template(template)) = template {
                result.push(template);
            } else if let Some(ParseElements::Page(cms_page)) = template {
                let name = cms_page
                    .templates
                    .iter()
                    .filter_map(|x| x.get_name())
                    .next();
                if let Some(name) = name {
                    pages.insert(name.to_string(), cms_page);
                }
            }
        }
    }
    (result, pages)
}

pub fn parse_file(generation_dirs: &impl GenerationDirs) -> Result<CMSSite, std::io::Error> {
    let file_path = &generation_dirs.in_source("index.cms");
    let contents = std::fs::read_to_string(file_path)?;
    let (templates, pages) = parse_templates(&contents, generation_dirs);
    Ok(CMSSite {
        original_content: contents,
        templates,
        pages,
    })
}

#[cfg(test)]
mod test {

    use crate::types::generation_dirs::TempGenerationDirs;

    use super::*;
    #[test]
    fn test_parse_templates() {
        const CONTENT: &str = r#"
        {{Title|hi}}
        {{Image|sample.jpg}}
        {{Navbar|bla,bla}}
        {{Links|Github:bla}}
        {{NKR-CMS-INFO}}
        {{Nonsense}}
        {{Page|
        {{Name|TestPage}}
        {{Paragraph|hi}}
        }}
        "#;
        let generation_dirs = TempGenerationDirs::default();
        let (templates, pages) = parse_templates(CONTENT, &generation_dirs);
        assert_eq!(templates.len(), 5);
        assert_eq!(pages.len(), 1);
        let (templates, pages) = parse_templates("{{broken_content", &generation_dirs);
        assert_eq!(templates.len(), 0);
        assert_eq!(pages.len(), 0);
    }
}
