mod expressions;
mod first_pass;
mod generator;
mod second_pass;
mod utils;

use first_pass::FirstPass;
use generator::Generator;
use second_pass::SecondPass;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let text = std::fs::read_to_string("./site/start.txt").unwrap();
    let text = text.graphemes(true).collect::<Vec<_>>();
    let parser = FirstPass::new(text);
    if !parser.errors.is_empty() {
        panic!("{:?}", parser.errors);
    }
    let expressions = parser.parse();
    let parser = SecondPass::new(expressions);
    let site = parser.parse().unwrap();
    let generator = Generator::new(site);
    let files = generator.generate();
    for file in files {
        let name = file.name.to_ascii_lowercase();
        let name = name.replace(" ", "-");
        let name = format!("{}.html", name);
        if !std::fs::exists("./output").unwrap() {
            std::fs::create_dir("./output").unwrap();
        }
        std::fs::write(format!("./output/{}", name), file.content).unwrap();
    }
}

#[cfg(test)]
mod tests {}
