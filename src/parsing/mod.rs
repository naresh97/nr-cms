mod get_tags;
mod parse_template_elements;

use std::collections::HashMap;

use crate::{
    run_args,
    types::{cms_page::CMSPage, cms_site::CMSSite, template_type::TemplateType},
};

use self::{get_tags::get_tags, parse_template_elements::*};

enum TemplateOrPage {
    Template(TemplateType),
    Page(CMSPage),
}

fn parse_page(content: Option<&str>, run_args: &run_args::RunArgs) -> Option<CMSPage> {
    let content = content?;
    let (templates, _pages) = parse_templates(content, run_args);
    Some(CMSPage { templates })
}

fn parse_template(template_content: &str, run_args: &run_args::RunArgs) -> Option<TemplateOrPage> {
    let template_separator = template_content.match_indices("|").next().map(|x| x.0);
    let (template_name, template_content) = match template_separator {
        Some(template_separator) => (
            template_content.get(0..template_separator)?,
            template_content.get(template_separator + 1..),
        ),
        _ => (template_content, None),
    };
    match template_name {
        "Navbar" => parse_navbar(template_content).map(|x| TemplateOrPage::Template(x)),
        "Title" => parse_title(template_content).map(|x| TemplateOrPage::Template(x)),
        "Paragraph" => parse_paragraph(template_content).map(|x| TemplateOrPage::Template(x)),
        "Links" => parse_links(template_content).map(|x| TemplateOrPage::Template(x)),
        "NKR-CMS-INFO" => parse_nkr_cms_info().map(|x| TemplateOrPage::Template(x)),
        "Image" => parse_image(template_content, run_args).map(|x| TemplateOrPage::Template(x)),
        "Name" => parse_name(template_content).map(|x| TemplateOrPage::Template(x)),
        "Page" => parse_page(template_content, run_args).map(|x| TemplateOrPage::Page(x)),
        _ => None,
    }
}

fn parse_templates(
    content: &str,
    run_args: &run_args::RunArgs,
) -> (Vec<TemplateType>, HashMap<String, CMSPage>) {
    let mut result: Vec<TemplateType> = Vec::new();
    let mut pages: HashMap<String, CMSPage> = HashMap::new();
    let tags = get_tags(content);
    if let Some(tags) = tags {
        for template_content in tags {
            let template = parse_template(template_content, run_args);
            if let Some(TemplateOrPage::Template(template)) = template {
                result.push(template);
            } else if let Some(TemplateOrPage::Page(cms_page)) = template {
                let name = cms_page
                    .templates
                    .iter()
                    .filter_map(|x| x.get_name())
                    .next();
                if let Some(name) = name {
                    pages.insert(String::from(name), cms_page);
                }
            }
        }
    }
    (result, pages)
}

pub fn parse_file(run_args: &run_args::RunArgs) -> Result<CMSSite, std::io::Error> {
    let file_path = &run_args.in_source("index.cms");
    let contents = std::fs::read_to_string(file_path)?;
    let (templates, pages) = parse_templates(&contents, run_args);
    Ok(CMSSite {
        original_content: contents,
        templates,
        pages,
    })
}

#[cfg(test)]
mod test {
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
        let run_args = run_args::RunArgs {
            generation_dir: String::from("gen/"),
            source_dir: String::from("sample/"),
            max_log_level: None,
        };
        let (templates, pages) = parse_templates(CONTENT, &run_args);
        assert_eq!(templates.len(), 5);
        assert_eq!(pages.len(), 1);
        let (templates, pages) = parse_templates("{{broken_content", &run_args);
        assert_eq!(templates.len(), 0);
        assert_eq!(pages.len(), 0);
    }
}
