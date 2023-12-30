use crate::{
    assets, run_args,
    types::{link_type::LinkType, template_type::TemplateType},
};

pub fn gen_title(templates: &Vec<TemplateType>) -> &str {
    let title = templates.iter().filter_map(|x| x.get_title()).next();
    match title {
        Some(title) => title,
        _ => "",
    }
}

pub fn gen_navbar(templates: &Vec<TemplateType>) -> String {
    let navbar = templates.iter().filter_map(|x| x.get_navbar()).next();
    match navbar {
        Some(paths) => {
            let paths: Vec<String> = paths
                .iter()
                .map(|x| format!("<a href=\"?page={x}\">{x}</a>"))
                .collect();
            let paths = paths.join(" | ");
            format!("<p>{paths}</p>")
        }
        _ => String::new(),
    }
}

pub fn gen_paragraphs(templates: &Vec<TemplateType>) -> String {
    let paragraphs = templates
        .iter()
        .filter_map(|x| x.get_paragraph())
        .collect::<Vec<_>>();
    let paragraphs = paragraphs
        .iter()
        .map(|x| format!("<p>{x}</p>"))
        .collect::<Vec<_>>();
    paragraphs.join("\n")
}

pub fn gen_links(templates: &Vec<TemplateType>) -> String {
    let links = templates
        .iter()
        .filter_map(|x| x.get_links())
        .collect::<Vec<_>>();
    if links.len() == 0 {
        return String::new();
    }
    let links = links[0];
    let links = links
        .iter()
        .map(|x| match x.0 {
            LinkType::Github => format!(
                r#"<a href="https://github.com/{}/">{} Github</a>"#,
                x.1,
                assets::svg_images::INVERTOCAT_SVG
            ),
        })
        .collect::<Vec<_>>();
    let links = links.join(" | ");
    return format!("<p>{links}</p>");
}

pub fn gen_nr_cms_info(templates: &Vec<TemplateType>) -> String {
    let info = templates.iter().filter_map(|x| x.get_nr_cms_info()).next();
    match info {
        Some(info) => {
            let info = format!("<p>{}</p>", info);
            return info;
        }
        _ => String::new(),
    }
}

pub fn gen_image(templates: &Vec<TemplateType>, run_args: &run_args::RunArgs) -> String {
    let image = templates.iter().filter_map(|x| x.get_image()).next();
    if let Some(image) = image {
        if *image.1 {
            let result = match image.2 {
                Some(x) => run_args.copy_asset_img(image.0, *x),
                _ => run_args.copy_asset(image.0),
            };
            if result.is_err() {
                return String::new();
            }
        }
        return format!(r#"<p><img src="{}"/></p>"#, image.0);
    }
    String::new()
}
