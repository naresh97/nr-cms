use crate::cms_types::{CMSFile, LinkType, TemplateType};

fn parse_title(content: &str) -> Option<TemplateType> {
    Some(TemplateType::Title { title: String::from(content) })
}

fn parse_paragraph(content: &str) -> Option<TemplateType> {
    Some(TemplateType::Paragraph { content: String::from(content) })
}

fn parse_links(content: &str) -> Option<TemplateType> {
    let link_pairs: Vec<&str> = content.split(",").collect();
    let link_pairs: Vec<(LinkType, String)> = link_pairs.iter().map(|x| {
        let pair: Vec<&str> = x.split(":").collect();
        if pair.len() == 2 {
            let link_type = match pair[0] {
                "Github" => Some(LinkType::Github),
                _ => None
            };
            if link_type.is_none() {
                return None;
            }
            let link_type = link_type.unwrap();
            let pair = (link_type, String::from(pair[1]));
            return Some(pair);
        }
        return None;
    }).filter(|x| x.is_some()).map(|x| x.unwrap()).collect::<Vec<(LinkType, String)>>();
    let link_pairs: std::collections::HashMap<_, _> = link_pairs.into_iter().collect();
    if link_pairs.len() > 0 {
        return Some(TemplateType::Links { links: link_pairs });
    }
    return None;
}

fn parse_navbar(content: &str) -> Option<TemplateType> {
    let paths: Vec<std::path::PathBuf> = content.split(",").map(|x| {
        let mut new_path = std::path::PathBuf::new();
        new_path.push("./");
        new_path.push(x);
        new_path
    }).collect();
    Some(TemplateType::Navbar {
        paths
    })
}

fn parse_template(template_content: &str) -> Option<TemplateType> {
    let template_name_content: Vec<&str> = template_content.split("|").collect();
    if template_name_content.len() != 2 {
        return None;
    }
    let template_name = template_name_content[0];
    let template_content = template_name_content[1];
    match &template_name {
        &"Navbar" => parse_navbar(template_content),
        &"Title" => parse_title(template_content),
        &"Paragraph" => parse_paragraph(template_content),
        &"Links" => parse_links(template_content),
        &"NKR-CMS-INFO" => parse_nkr_cms_info(),
        _ => None
    }
}

fn parse_nkr_cms_info() -> Option<TemplateType> {
    return Some(TemplateType::NRCMSInfo {
        text: "This website is made with <a href=\"https://github.com/naresh97/nr-cms\">NR-CMS.</a>"
    });
}

pub fn parse_templates(cms_file: &mut CMSFile) {
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
        let template = parse_template(template_content);
        if template.is_some() {
            cms_file.templates.push(template.unwrap());
        }
    }
}
