use crate::{
    cms_types::{CMSFile, LinkType, TemplateType},
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
        .map(|x| {
            let pair: Vec<&str> = x.split(":").collect();
            if pair.len() == 2 {
                let link_type = match pair[0] {
                    "Github" => Some(LinkType::Github),
                    _ => None,
                };
                if link_type.is_none() {
                    return None;
                }
                let link_type = link_type.unwrap();
                let pair = (link_type, String::from(pair[1]));
                return Some(pair);
            }
            return None;
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
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

pub fn parse_templates(cms_file: &mut CMSFile, run_args: &run_args::RunArgs) {
    let original_content = &cms_file.original_content;
    let mut template_opening = original_content.match_indices("{{");
    let mut template_closing = original_content.match_indices("}}");
    loop {
        let opening = template_opening.next();
        let closing = template_closing.next();
        if opening == None && closing == None {
            break;
        }
        let opening = opening.unwrap().0;
        let closing = closing.unwrap().0;
        let template_content = &original_content[opening + 2..closing];
        let template = parse_template(template_content, run_args);
        if template.is_some() {
            cms_file.templates.push(template.unwrap());
        }
    }
}
