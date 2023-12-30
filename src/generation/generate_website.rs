use crate::{assets, run_args, types::cms_site::CMSSite};

use super::{page_generator::*, template_generators::*};

pub fn generate_website(cms_site: &CMSSite, run_args: &run_args::RunArgs) -> String {
    let templates = &cms_site.templates;
    let pages = &cms_site.pages;
    let title = gen_title(templates);
    let navbar = gen_navbar(templates);
    let nr_cms_info = gen_nr_cms_info(templates);
    let style = assets::styles::SITE_STYLE;
    let script = assets::scripts::PAGE_LOGIC;
    let pages = gen_pages(pages, run_args);
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
