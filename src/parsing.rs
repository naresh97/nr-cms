use std::collections::HashMap;

use crate::{
    cms_types::{CMSPage, LinkType, TemplateType},
    img_handling::{get_img_as_b64_url, get_img_b64_size},
    run_args,
};

fn parse_title(content: Option<&str>) -> Option<TemplateType> {
    let content = content?;
    Some(TemplateType::Title {
        title: String::from(content),
    })
}

fn parse_paragraph(content: Option<&str>) -> Option<TemplateType> {
    let content = content?;
    Some(TemplateType::Paragraph {
        content: String::from(content),
    })
}

fn parse_links(content: Option<&str>) -> Option<TemplateType> {
    let content = content?;
    let link_pairs: Vec<&str> = content.split(",").collect();
    let link_pairs: Vec<(LinkType, String)> = link_pairs
        .iter()
        .filter_map(|x| {
            let pair: Vec<&str> = x.split(":").collect();
            if pair.len() == 2 {
                let link_type = match pair[0] {
                    "Github" => Some(LinkType::Github),
                    _ => None,
                };
                if let Some(link_type) = link_type {
                    let pair = (link_type, String::from(pair[1]));
                    return Some(pair);
                }
                return None;
            }
            return None;
        })
        .collect::<Vec<(LinkType, String)>>();
    let link_pairs: std::collections::HashMap<_, _> = link_pairs.into_iter().collect();
    if link_pairs.len() > 0 {
        return Some(TemplateType::Links { links: link_pairs });
    }
    return None;
}

fn parse_navbar(content: Option<&str>) -> Option<TemplateType> {
    let content = content?;
    let paths = content
        .split(",")
        .map(|x| String::from(x))
        .collect::<Vec<_>>();
    Some(TemplateType::Navbar { paths })
}

fn parse_image(content: Option<&str>, run_args: &run_args::RunArgs) -> Option<TemplateType> {
    let content = content?;
    let args = content.split(",").collect::<Vec<_>>();
    let mut url = String::from(*args.get(0)?);
    let size = args.get(1);
    let size = match size {
        Some(x) => str::parse::<u32>(*x).ok(),
        _ => None,
    };
    let source_url = run_args.in_source(&url);
    let b64_size = get_img_b64_size(&source_url.as_path(), size).ok()?;
    const MAXIMUM_B64_SIZE: usize = 1000;
    let mut copy_asset = true;
    if b64_size <= MAXIMUM_B64_SIZE {
        url = get_img_as_b64_url(&source_url.as_path(), size).ok()?;
        copy_asset = false;
    }
    Some(TemplateType::Image {
        url,
        copy_asset,
        size,
    })
}

fn parse_name(content: Option<&str>) -> Option<TemplateType> {
    Some(TemplateType::Name {
        name: String::from(content?),
    })
}

fn parse_page(content: Option<&str>, run_args: &run_args::RunArgs) -> Option<CMSPage> {
    let content = content?;
    let (templates, pages) = parse_templates(content, run_args);
    Some(CMSPage { templates })
}

enum TemplateOrPage {
    Template(TemplateType),
    Page(CMSPage),
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

fn parse_nkr_cms_info() -> Option<TemplateType> {
    return Some(TemplateType::NRCMSInfo {
        text:
            "This website is made with <a href=\"https://github.com/naresh97/nr-cms\">NR-CMS.</a>",
    });
}

fn get_tags(content: &str) -> Option<Vec<&str>> {
    const OPENING_BRACE: &str = "{{";
    const CLOSING_BRACE: &str = "}}";
    let mut braces_opening = content.match_indices(OPENING_BRACE).collect::<Vec<_>>();
    let mut braces_closing = content.match_indices(CLOSING_BRACE).collect::<Vec<_>>();
    braces_opening.append(&mut braces_closing);
    let mut braces = braces_opening;
    braces.sort_by(|a, b| a.0.cmp(&b.0));
    let mut templates: Vec<(usize, usize)> = Vec::new();
    let mut scope_count = 0;
    let mut current_template: (usize, usize) = (0, 0);
    for b in braces {
        let index = b.0;
        let braces = b.1;
        match braces {
            OPENING_BRACE => {
                if scope_count == 0 {
                    current_template.0 = index + 2;
                }
                scope_count += 1;
            }
            CLOSING_BRACE => {
                scope_count -= 1;
                if scope_count == 0 {
                    current_template.1 = index;
                    templates.push(current_template);
                }
            }
            _ => (),
        };
    }
    if scope_count != 0 {
        log::debug!("{content}");
        log::error!("Opening/Closing tags mismatch.");
        return None;
    }
    Some(
        templates
            .iter()
            .map(|x| {
                let x = &content[x.0..x.1];
                return x;
            })
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_tags() {
        let test = "{{hello}} {{world}} {{outer {{inner}}}}";
        let tags = get_tags(test);
        assert!(tags.is_some());
        if let Some(tags) = tags {
            assert_eq!(tags.len(), 3);
            assert_eq!(tags[0], "hello");
            assert_eq!(tags[1], "world");
            assert_eq!(tags[2], "outer {{inner}}");
        }

        let test = "{{hello}} {{there";
        let tags = get_tags(test);
        assert!(tags.is_none());
    }
}

pub fn parse_templates(
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
