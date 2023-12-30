use crate::{run_args, types::cms_site::CMSSite};

use super::template_generators::*;

pub fn gen_pages(cms_site: &CMSSite, run_args: &run_args::RunArgs) -> String {
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
