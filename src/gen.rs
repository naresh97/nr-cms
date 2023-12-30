use crate::{
    assets, run_args,
    types::{cms_site::CMSSite, link_type::LinkType, template_type::TemplateType},
};

fn gen_title(cms_site: &CMSSite) -> &str {
    let title = cms_site
        .templates
        .iter()
        .filter_map(|x| x.get_title())
        .next();
    match title {
        Some(title) => title,
        _ => "",
    }
}

fn gen_navbar(cms_site: &CMSSite) -> String {
    let navbar = cms_site
        .templates
        .iter()
        .filter_map(|x| x.get_navbar())
        .next();
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

fn gen_paragraphs(templates: &Vec<TemplateType>) -> String {
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

fn gen_links(templates: &Vec<TemplateType>) -> String {
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

fn gen_nr_cms_info(cms_site: &CMSSite) -> String {
    let info = cms_site
        .templates
        .iter()
        .filter_map(|x| x.get_nr_cms_info())
        .next();
    match info {
        Some(info) => {
            let info = format!("<p>{}</p>", info);
            return info;
        }
        _ => String::new(),
    }
}

fn gen_image(templates: &Vec<TemplateType>, run_args: &run_args::RunArgs) -> String {
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

fn gen_pages(cms_site: &CMSSite, run_args: &run_args::RunArgs) -> String {
    let pages = &cms_site.pages;
    let mut pages_string = String::new();
    for (name, page) in pages {
        let templates = &page.templates;
        let paragraphs = gen_paragraphs(templates);
        let links = gen_links(templates);
        let image = gen_image(templates, run_args);

        let page_string = format!(
            r#"
        <div id="page-{name}" class="page">
        {image}
        {paragraphs}
        {links}
        </div>
        "#
        );
        pages_string.push_str(&page_string);
    }
    pages_string
}

pub fn generate_website(cms_site: &CMSSite, run_args: &run_args::RunArgs) -> String {
    let title = gen_title(cms_site);
    let navbar = gen_navbar(cms_site);
    let nr_cms_info = gen_nr_cms_info(cms_site);
    let style = assets::styles::SITE_STYLE;
    let script = assets::scripts::PAGE_LOGIC;
    let pages = gen_pages(cms_site, run_args);
    let site = format!(
        r#"
    <html>
    <head>
    {script}
    {style}
    <title>{title}</title>
    </head>
    <body>
    <h1>{title}</h1>
    {navbar}
    {pages}
    {nr_cms_info}
    </body>
    </html>
    "#
    );
    return site;
}
