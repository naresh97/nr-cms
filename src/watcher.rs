use std::path::Path;

use notify::{RecursiveMode, Watcher};

use crate::cms_types::CMSFile;
use crate::gen::generate_website;
use crate::parsing;

fn load_cms_file(file_path: &str) -> Result<CMSFile, std::io::Error> {
    println!("Loading CMS files");
    let file_path = Path::new(file_path);
    let contents = std::fs::read_to_string(file_path)?;
    let mut cms_file = CMSFile {
        original_content: contents,
        templates: Vec::new(),
    };
    parsing::parse_templates(&mut cms_file);
    return Ok(cms_file);
}

fn write_gen_site(file_path: &str, cms_file: &CMSFile) {
    println!("Writing generated files");
    let gen_file = generate_website(&cms_file);
    let file_path = Path::new(file_path);
    let parent = file_path.parent().expect("Could not get parent directory");
    std::fs::create_dir_all(parent).expect("Could not create necessary directories");
    std::fs::write(file_path, gen_file).expect("could not write generated file");
}

fn watch_event(_event: notify::Event) {
    println!("Filesystem change detected");
    let index_file = load_cms_file("./sample/index.cms").expect("Could not load index.cms");
    write_gen_site("./gen/index.html", &index_file);
}

fn watch_error(_e: notify::Error) {}

pub fn watch() {
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
            Ok(event) => watch_event(event),
            Err(e) => watch_error(e)
        }
    }).unwrap();
    watcher.watch(Path::new("./sample"), RecursiveMode::Recursive).unwrap();
    loop {};
}