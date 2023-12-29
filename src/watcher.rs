use notify::{RecursiveMode, Watcher};

use crate::cms_types::CMSSite;
use crate::gen::generate_website;
use crate::{parsing, run_args};

fn load_cms_site(
    file_path: std::path::PathBuf,
    run_args: &run_args::RunArgs,
) -> Result<CMSSite, std::io::Error> {
    println!("Loading CMS files");
    let contents = std::fs::read_to_string(file_path)?;
    let mut cms_site = CMSSite {
        original_content: contents,
        templates: Vec::new(),
    };
    parsing::parse_templates(&mut cms_site, run_args);
    return Ok(cms_site);
}

fn write_gen_site(file_path: std::path::PathBuf, cms_site: &CMSSite, run_args: &run_args::RunArgs) {
    println!("Writing generated files");
    let gen_file = generate_website(&cms_site, run_args);
    let parent = file_path.parent().expect("Could not get parent directory");
    std::fs::create_dir_all(parent).expect("Could not create necessary directories");
    std::fs::write(file_path, gen_file).expect("could not write generated file");
}

fn watch_event(_event: notify::Event, run_args: run_args::RunArgs) {
    println!("Filesystem change detected");
    let index_file = load_cms_site(run_args.in_source("index.cms"), &run_args)
        .expect("Could not load index.cms");
    write_gen_site(run_args.in_gen("index.html"), &index_file, &run_args);
}

fn watch_error(_e: notify::Error) {}

pub fn watch(run_args: run_args::RunArgs) -> Result<(), notify::Error> {
    let source_dir = &run_args.source_dir.clone();
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => watch_event(event, run_args.clone()),
        Err(e) => watch_error(e),
    })?;
    watcher.watch(std::path::Path::new(source_dir), RecursiveMode::Recursive)?;
    loop {}
}
