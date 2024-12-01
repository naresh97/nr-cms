#![allow(dead_code)]
mod expressions;
mod parser;
mod utils;

use parser::Parser;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let text = std::fs::read_to_string("./site/start.txt").unwrap();
    let text = text.graphemes(true).collect::<Vec<_>>();
    let parser = Parser::new(text);
    let site = parser.parse();
    println!("{:?}", site);
}

#[cfg(test)]
mod tests {}
