use crate::{
    assets, run_args,
    types::{
        link_type::LinkType,
        template_type::{TemplateType, TemplateTypeVector},
    },
};

pub fn gen_title(templates: &Vec<TemplateType>) -> &str {
    match templates.get_title() {
        Some(title) => title,
        _ => "",
    }
}

pub fn gen_navbar(templates: &Vec<TemplateType>) -> String {
    match templates.get_navbar() {
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
        .get_paragraphs()
        .iter()
        .map(|x| format!("<p>{x}</p>"))
        .collect::<Vec<_>>();
    paragraphs.join("\n")
}

pub fn gen_links(templates: &Vec<TemplateType>) -> String {
    match templates.get_links() {
        Some(links) => {
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
            format!("<p>{links}</p>")
        }
        _ => String::new(),
    }
}

pub fn gen_nr_cms_info(templates: &Vec<TemplateType>) -> String {
    match templates.get_nr_cms_info() {
        Some(info) => {
            let info = format!("<p>{}</p>", info);
            return info;
        }
        _ => String::new(),
    }
}

pub fn gen_image(templates: &Vec<TemplateType>, run_args: &run_args::RunArgs) -> String {
    match templates.get_image() {
        Some(image) => {
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
        _ => String::new(),
    }
}
