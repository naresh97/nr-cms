use crate::{
    img_handling::{get_img_as_b64_url, get_img_b64_size},
    run_args,
    types::{link_type::LinkType, template_type::TemplateType},
};

pub fn parse_title(content: Option<&str>) -> Option<TemplateType> {
    let content = content?;
    Some(TemplateType::Title {
        title: content.to_string(),
    })
}

pub fn parse_paragraph(content: Option<&str>) -> Option<TemplateType> {
    let content = content?;
    Some(TemplateType::Paragraph {
        content: content.to_string(),
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
                    let pair = (link_type, pair[1].to_string());
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
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    Some(TemplateType::Navbar { paths })
}

pub fn parse_image(content: Option<&str>, run_args: &run_args::RunArgs) -> Option<TemplateType> {
    let content = content?;
    let args = content.split(",").collect::<Vec<_>>();
    let mut url = args.get(0)?.to_string();
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
        name: content?.to_string(),
    })
}

pub fn parse_nkr_cms_info() -> Option<TemplateType> {
    return Some(TemplateType::NRCMSInfo {
        text:
            "This website was automatically generated with <a href=\"https://github.com/naresh97/nr-cms\">NR-CMS.</a>",
    });
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_simple_parsing() {
        const TEST: &str = "test";
        assert_eq!(parse_name(Some(TEST)).unwrap().get_name().unwrap(), TEST);
        assert_eq!(
            parse_paragraph(Some(TEST))
                .unwrap()
                .get_paragraph()
                .unwrap(),
            TEST
        );
        assert_eq!(parse_title(Some(TEST)).unwrap().get_title().unwrap(), TEST);
        assert!(parse_nkr_cms_info().unwrap().get_nr_cms_info().is_some());
    }
    #[test]
    fn test_parse_links() {
        const LINKS: &str = "Github:A,None:B";
        let links = parse_links(Some(LINKS)).unwrap();
        let links = links.get_links().unwrap();
        assert!(links.contains_key(&LinkType::Github));
        assert_eq!(links.get(&LinkType::Github).unwrap(), "A");
        const LINKS_BROKEN: &str = "GithubA,NoneB";
        let links = parse_links(Some(LINKS_BROKEN));
        assert!(links.is_none());
    }

    #[test]
    fn test_parse_navbar() {
        const PAGES: &str = "a,b,c";
        let pages = parse_navbar(Some(PAGES)).unwrap();
        let pages = pages.get_navbar().unwrap();
        assert_eq!(pages.len(), 3);
    }

    #[test]
    fn test_parse_image() {
        const IMG: &str = "sample.jpg";
        let run_args = run_args::RunArgs {
            generation_dir: "gen/".to_string(),
            source_dir: "sample/".to_string(),
            max_log_level: Default::default(),
            watch: Default::default(),
        };
        let image = parse_image(Some(IMG), &run_args).unwrap();
        let image = image.get_image().unwrap();
        assert!(image.1);
        assert_eq!(image.0, IMG);
        const IMG_SIZE: &str = "sample.jpg,10";
        let image = parse_image(Some(IMG_SIZE), &run_args).unwrap();
        let image = image.get_image().unwrap();
        assert_eq!(image.2.unwrap(), 10);
        assert!(!image.1);
    }
}
