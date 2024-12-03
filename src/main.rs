#![warn(clippy::pedantic)]
#![deny(clippy::unused_result_ok)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

mod expression_parser;
mod expressions;
mod site_builder;
mod site_generator;
mod utils;

use expression_parser::ExpressionParser;
use site_builder::SiteBuilder;
use site_generator::SiteGenerator;
use unicode_segmentation::UnicodeSegmentation;

#[allow(clippy::restriction)]
fn main() {
    let text = std::fs::read_to_string("./site/start.txt").unwrap();
    let text = text.graphemes(true).collect::<Vec<_>>();
    let parser = ExpressionParser::new(text);
    assert!(parser.errors.is_empty(), "{:?}", parser.errors);
    let expressions = parser.parse();
    let parser = SiteBuilder::new(expressions);
    let site = parser.parse().unwrap();
    let generator = SiteGenerator::new(site);
    let files = generator.generate();
    for file in files {
        let name = file.name.to_ascii_lowercase();
        let name = name.replace(' ', "-");
        let name = format!("{name}.html");
        if !std::fs::exists("./output").unwrap() {
            std::fs::create_dir("./output").unwrap();
        }
        std::fs::write(format!("./output/{name}"), file.content).unwrap();
    }
}

#[cfg(test)]
mod tests {}
