enum TemplateType{
    Navbar{
        paths: Vec<String>
    }
}

struct CMSFile{
    original_content: String,
    templates: Vec<TemplateType>
}

fn parse_navbar(content: &str) -> Option<TemplateType> {
    
    return None;
}

fn parse_template(template_content: &str) -> Option<TemplateType>{
    let template_name_index = template_content.find("|");
    if template_name_index.is_none(){
        return None;
    }
    let template_name_index = template_name_index.unwrap();
    let template_name = &template_content[0..template_name_index];
    let template_content = &template_content[template_name_index+1..];
    match &template_name{
        &"Navbar" => parse_navbar(template_content),
        _ => None
    }
}

fn parse_templates(cms_file : &mut CMSFile){
    let original_content = &cms_file.original_content;
    let mut template_opening = original_content.match_indices("{{");
    let mut template_closing = original_content.match_indices("}}");
    loop{
        let opening = template_opening.next();
        let closing = template_closing.next();
        if opening == None && closing == None{
            break;
        }
        let opening = opening.unwrap().0;
        let closing = closing.unwrap().0;
        let template_content = &original_content[opening+2..closing];
        let template = parse_template(template_content);
        if template.is_some(){
            cms_file.templates.push(template.unwrap());
        }
    }
}

fn load_cms_file(file_path: &str) -> Result<CMSFile, std::io::Error> {
    let file_path = std::path::Path::new(file_path);
    let contents = std::fs::read_to_string(file_path)?;
    let mut cms_file = CMSFile{
        original_content: contents,
        templates: Vec::new()
    };
    parse_templates(&mut cms_file);
    return Ok(cms_file);
}

fn main() {
    let index_file = load_cms_file("./sample/index.cms").expect("Could not load index.cms");
}
