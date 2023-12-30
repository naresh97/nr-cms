use crate::{
    img_handling::{get_img_as_b64_url, get_img_b64_size},
    run_args,
    types::{link_type::LinkType, template_type::TemplateType},
};

pub fn parse_title(content: Option<&str>) -> Option<TemplateType> {
    let content = content?;
    Some(TemplateType::Title {
        title: String::from(content),
    })
}

pub fn parse_paragraph(content: Option<&str>) -> Option<TemplateType> {
    let content = content?;
    Some(TemplateType::Paragraph {
        content: String::from(content),
    })
}

pub fn parse_links(content: Option<&str>) -> Option<TemplateType> {
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

pub fn parse_navbar(content: Option<&str>) -> Option<TemplateType> {
    let content = content?;
    let paths = content
        .split(",")
        .map(|x| String::from(x))
        .collect::<Vec<_>>();
    Some(TemplateType::Navbar { paths })
}

pub fn parse_image(content: Option<&str>, run_args: &run_args::RunArgs) -> Option<TemplateType> {
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

pub fn parse_name(content: Option<&str>) -> Option<TemplateType> {
    Some(TemplateType::Name {
        name: String::from(content?),
    })
}

pub fn parse_nkr_cms_info() -> Option<TemplateType> {
    return Some(TemplateType::NRCMSInfo {
        text:
            "This website is made with <a href=\"https://github.com/naresh97/nr-cms\">NR-CMS.</a>",
    });
}
