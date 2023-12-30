use crate::{assets, parsing, run_args, types::cms_site::CMSSite};

use super::{page_generator::*, template_generators::*};

pub fn load_and_write_site(run_args: &run_args::RunArgs) {
    let index_file = load_cms_site(run_args.in_source("index.cms"), &run_args);
    match index_file {
        Ok(index_file) => {
            let result = write_gen_site(run_args.in_gen("index.html"), &index_file, &run_args);
            if let Err(e) = result {
                log::error!("Could not generate site: {}", e.to_string());
            }
        }
        Err(e) => {
            log::error!("Could not load CMS site: {}", e.to_string());
        }
    };
}

fn load_cms_site(
    file_path: std::path::PathBuf,
    run_args: &run_args::RunArgs,
) -> Result<CMSSite, std::io::Error> {
    println!("Loading CMS files");
    let contents = std::fs::read_to_string(file_path)?;
    let (templates, pages) = parsing::parse_templates(&contents, run_args);
    let cms_site = CMSSite {
        original_content: contents,
        templates,
        pages,
    };

    return Ok(cms_site);
}

fn write_gen_site(
    file_path: std::path::PathBuf,
    cms_site: &CMSSite,
    run_args: &run_args::RunArgs,
) -> Result<(), std::io::Error> {
    log::info!("Generating website");
    let gen_file = generate_website(&cms_site, run_args);
    let parent = file_path.parent().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not get parent",
    ))?;
    std::fs::create_dir_all(parent)?;
    std::fs::write(file_path, gen_file)?;
    return Ok(());
}

fn generate_website(cms_site: &CMSSite, run_args: &run_args::RunArgs) -> String {
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
