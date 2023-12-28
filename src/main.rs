use crate::cms_types::CMSFile;
use crate::gen::generate_website;

mod parsing;
mod cms_types;
mod gen;

fn load_cms_file(file_path: &str) -> Result<CMSFile, std::io::Error> {
    let file_path = std::path::Path::new(file_path);
    let contents = std::fs::read_to_string(file_path)?;
    let mut cms_file = CMSFile {
        original_content: contents,
        templates: Vec::new(),
    };
    parsing::parse_templates(&mut cms_file);
    return Ok(cms_file);
}

fn write_gen_site(file_path: &str, cms_file: &CMSFile) {
    let gen_file = generate_website(&cms_file);
    let file_path = std::path::Path::new(file_path);
    let parent = file_path.parent().expect("Could not get parent directory");
    std::fs::create_dir_all(parent).expect("Could not create necessary directories");
    std::fs::write(file_path, gen_file).expect("could not write generated file");
}

fn main() {
    let index_file = load_cms_file("./sample/index.cms").expect("Could not load index.cms");
    write_gen_site("./gen/index.html", &index_file);
}
