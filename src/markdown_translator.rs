use crate::utils::generate_page_filename;

pub struct MarkdownTranslator {
    current: usize,
    start: usize,
    text: Vec<char>,
    output: Vec<char>,
    errors: Vec<MarkdownTranslatorError>,
}
pub struct MarkdownTranslatorError {
    text: String,
    message: String,
}
impl std::fmt::Display for MarkdownTranslatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Markdown Error: {}\n\n{}here", self.message, self.text)
    }
}

impl MarkdownTranslator {
    pub fn new(markdown: &str) -> Self {
        let text = markdown.chars().collect::<Vec<_>>();
        MarkdownTranslator {
            start: 0,
            current: 0,
            output: Vec::with_capacity(text.len()),
            text,
            errors: Vec::new(),
        }
    }
    pub fn translate(mut self) -> (String, Vec<MarkdownTranslatorError>) {
        while self.current != self.text.len() {
            self.start = self.current;
            self.translate_next();
        }
        (self.output.into_iter().collect(), self.errors)
    }

    fn translate_next(&mut self) {
        if self.peek_str("[[") {
            self.translate_page_link();
            return;
        }

        let c = self.advance();

        if c == '[' {
            while !self.is_at_end() && self.peek() != Some(']') {
                self.advance();
            }
            if self.is_at_end() {
                self.push_error("Unexpected end of line".to_string());
                return;
            }
            let link_text: String = self.text[self.start + 1..self.current].iter().collect();
            self.advance();
            if self.advance() != '(' {
                self.push_error("Expected '('".to_string());
                return;
            }
            let start = self.current;
            while !self.is_at_end() && self.peek() != Some(')') {
                self.advance();
            }
            let link_url: String = self.text[start..self.current].iter().collect();
            self.advance();
            let text = format!("<a href=\"{link_url}\">{link_text}</a>");
            self.push_text(&text);
            return;
        }

        self.output.push(c);
    }

    fn translate_page_link(&mut self) {
        while !self.is_at_end() && !self.peek_str("]]") {
            self.advance();
        }
        let page_name: String = self.text[self.start + 2..self.current].iter().collect();
        self.advance();
        self.advance();
        let filename = generate_page_filename(&page_name);
        let text = format!("<a href=\"{filename}\">{page_name}</a>");
        self.push_text(&text);
    }
}

impl MarkdownTranslator {
    fn advance(&mut self) -> char {
        let result = self.text[self.current];
        self.current += 1;
        result
    }
    fn peek(&self) -> Option<char> {
        self.text.get(self.current).copied()
    }
    fn peek_str(&self, pattern: &str) -> bool {
        let Some(text) = self.text.get(self.current..self.current + pattern.len()) else {
            return false;
        };
        let text = text.iter().copied();
        let pattern = pattern.chars();
        pattern.eq(text)
    }
    fn is_at_end(&self) -> bool {
        self.current == self.text.len()
    }
    fn push_text(&mut self, text: &str) {
        self.output.extend(text.chars());
    }
    fn push_error(&mut self, message: String) {
        self.errors.push(MarkdownTranslatorError {
            text: format!(
                "{}<---",
                self.text[self.start..self.current]
                    .iter()
                    .collect::<String>()
            ),
            message,
        });
    }
}
