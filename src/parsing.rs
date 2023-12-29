use crate::{
    cms_types::{CMSSite, LinkType, TemplateType},
    img_handling::{get_img_as_b64_url, get_img_b64_size},
    run_args,
};

fn parse_title(content: Option<&&str>) -> Option<TemplateType> {
    let content = *(content?);
    Some(TemplateType::Title {
        title: String::from(content),
    })
}

fn parse_paragraph(content: Option<&&str>) -> Option<TemplateType> {
    let content = *(content?);
    Some(TemplateType::Paragraph {
        content: String::from(content),
    })
}

fn parse_links(content: Option<&&str>) -> Option<TemplateType> {
    let content = *(content?);
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

fn parse_navbar(content: Option<&&str>) -> Option<TemplateType> {
    let content = *(content?);
    let paths: Vec<std::path::PathBuf> = content
        .split(",")
        .map(|x| {
            let mut new_path = std::path::PathBuf::new();
            new_path.push("./");
            new_path.push(x);
            new_path
        })
        .collect();
    Some(TemplateType::Navbar { paths })
}

fn parse_image(content: Option<&&str>, run_args: &run_args::RunArgs) -> Option<TemplateType> {
    let content = *(content?);
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

fn parse_template(template_content: &str, run_args: &run_args::RunArgs) -> Option<TemplateType> {
    let template_name_content: Vec<&str> = template_content.split("|").collect();
    if template_name_content.len() < 1 {
        return None;
    }
    let template_name = template_name_content[0];
    let template_content = template_name_content.get(1);
    match template_name {
        "Navbar" => parse_navbar(template_content),
        "Title" => parse_title(template_content),
        "Paragraph" => parse_paragraph(template_content),
        "Links" => parse_links(template_content),
        "NKR-CMS-INFO" => parse_nkr_cms_info(),
        "Image" => parse_image(template_content, run_args),
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

pub fn parse_templates(cms_site: &mut CMSSite, run_args: &run_args::RunArgs) {
    let tags = get_tags(&cms_site.original_content);
    if let Some(tags) = tags {
        for template_content in tags {
            let template = parse_template(template_content, run_args);
            if let Some(template) = template {
                cms_site.templates.push(template);
            }
        }
    }
}
