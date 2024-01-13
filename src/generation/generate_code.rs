use regex::Regex;

use crate::types::template_type::TemplateType;

struct Replacement {
    css_class: String,
    match_keywords: Vec<String>,
}

fn wrap_in_span(keyword: &str, css_class: &str) -> String {
    format!(r#"<span class="code-{css_class}">{keyword}</span>"#)
}

fn keyword_replacement(keyword: &str, haystack: &str, rep: &str) -> Option<String> {
    let re = Regex::new(&format!(r"\b{keyword}\b")).ok()?;
    let result = re.replace_all(haystack, rep);
    Some(result.to_string())
}

fn process_replacements(code: &str, replacements: &[Replacement]) -> String {
    let mut code = code.to_string();
    for replacement in replacements {
        for match_keyword in &replacement.match_keywords {
            let span = wrap_in_span(match_keyword, &replacement.css_class);
            code = keyword_replacement(match_keyword, &code, &span).unwrap_or(code);
        }
    }
    code
}

fn load_replacements() -> Vec<Replacement> {
    Vec::new()
}

#[macro_export]
macro_rules! string_vec {
    ( $( $x:expr ),* ) => {
        [$($x.to_string(),)*].to_vec()
    };
}

pub fn gen_code(template: &TemplateType) -> String {
    let replacements = load_replacements();
    let code = template
        .get_code()
        .map(|code| process_replacements(code, &replacements));
    let code = code.map(|x| format!("<pre><code>\n{x}\n</code></pre>"));
    code.unwrap_or_default()
}

#[cfg(test)]
mod test {
    use super::{process_replacements, Replacement};

    #[test]
    fn test_process_replacements() {
        let replacements = [Replacement {
            css_class: "keywords".into(),
            match_keywords: string_vec!["int", "true"],
        }];
        let code = r#"
            int main(){
                bool x = true;
                std::string y = "namewithtrue";
                return 0;
            }
        "#;
        let code = process_replacements(code, &replacements);
        print!("{}", code);
    }
}
