use crate::cms_types::{CMSFile, LinkType, TemplateType};
use crate::{assets, run_args};

fn gen_title(cms_file: &CMSFile) -> &str {
    let title = cms_file.templates.iter().find(|x| {
        if let TemplateType::Title { title: _title } = x {
            return true;
        }
        return false;
    });
    match title {
        Some(TemplateType::Title { title: m_title }) => m_title,
        _ => "",
    }
}

fn gen_navbar(cms_file: &CMSFile) -> String {
    let navbar = cms_file.templates.iter().find(|x| match x {
        TemplateType::Navbar { paths: _ } => true,
        _ => false,
    });
    match navbar {
        Some(TemplateType::Navbar { paths }) => {
            let paths: Vec<String> = paths
                .iter()
                .filter_map(|x| {
                    let path_str = x.to_str();
                    match path_str {
                        Some(path_str) => Some(format!("<a href=\"{path_str}\">{path_str}</a>")),
                        _ => None,
                    }
                })
                .collect();
            let paths = paths.join(" | ");
            format!("<p>{paths}</p>")
        }
        _ => String::new(),
    }
}

fn gen_paragraphs(cms_file: &CMSFile) -> String {
    let paragraphs = cms_file
        .templates
        .iter()
        .filter_map(|x| match x {
            TemplateType::Paragraph { content } => Some(content),
            _ => None,
        })
        .collect::<Vec<&String>>();
    let paragraphs = paragraphs
        .iter()
        .map(|x| format!("<p>{x}</p>"))
        .collect::<Vec<String>>();
    paragraphs.join("\n")
}

fn gen_links(cms_file: &CMSFile) -> String {
    let links = cms_file
        .templates
        .iter()
        .filter_map(|x| match x {
            TemplateType::Links { links } => Some(links),
            _ => None,
        })
        .collect::<Vec<_>>();
    if links.len() == 0 {
        return String::new();
    }
    let links = links[0];
    let links = links
        .iter()
        .map(|x| match x.0 {
            LinkType::Github => format!(r#"<a href="https://github.com/{}/">Github</a>"#, x.1),
        })
        .collect::<Vec<_>>();
    let links = links.join(" | ");
    return format!("<p>{links}</p>");
}

fn gen_nr_cms_info(cms_file: &CMSFile) -> String {
    let info = cms_file
        .templates
        .iter()
        .filter_map(|x| match x {
            TemplateType::NRCMSInfo { text } => Some(text),
            _ => None,
        })
        .collect::<Vec<_>>();
    if info.len() == 0 {
        return String::new();
    }
    let info = format!("<p>{}</p>", info[0]);
    return info;
}

fn gen_image(cms_file: &CMSFile, run_args: &run_args::RunArgs) -> String {
    let image = cms_file
        .templates
        .iter()
        .filter_map(|x| match x {
            TemplateType::Image { url, copy_asset } => Some((url, copy_asset)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let image = image.get(0);
    if image.is_none() {
        return String::new();
    }
    let image = image.unwrap();

    if *image.1 {
        let result = run_args.copy_asset(image.0);
        if result.is_err() {
            return String::new();
        }
    }

    format!(r#"<p><img src="{}"/></p>"#, image.0)
}

pub fn generate_website(cms_file: &CMSFile, run_args: &run_args::RunArgs) -> String {
    let title = gen_title(cms_file);
    let navbar = gen_navbar(cms_file);
    let paragraphs = gen_paragraphs(cms_file);
    let links = gen_links(cms_file);
    let nr_cms_info = gen_nr_cms_info(cms_file);
    let style = assets::styles::SITE_STYLE;
    let image = gen_image(cms_file, run_args);
    let site = format!(
        r#"
    <html>
    <head>
    {style}
    <title>{title}</title>
    </head>
    <body>
    <h1>{title}</h1>
    {image}
    {navbar}
    {paragraphs}
    {links}
    {nr_cms_info}
    </body>
    </html>
    "#
    );
    return site;
}
