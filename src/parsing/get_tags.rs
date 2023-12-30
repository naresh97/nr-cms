pub fn get_tags(content: &str) -> Option<Vec<&str>> {
    const OPENING_BRACE: &str = "{{";
    const CLOSING_BRACE: &str = "}}";
    let mut braces_opening = content.match_indices(OPENING_BRACE).collect::<Vec<_>>();
    let mut braces_closing = content.match_indices(CLOSING_BRACE).collect::<Vec<_>>();
    braces_opening.append(&mut braces_closing);
    let mut braces = braces_opening;
    braces.sort_by(|a, b| a.0.cmp(&b.0));
    let mut templates: Vec<(usize, usize)> = Vec::new();
    let mut scope_count = 0;
    let mut current_template: (usize, usize) = (0, 0);
    for b in braces {
        let index = b.0;
        let braces = b.1;
        if braces == OPENING_BRACE {
            if scope_count == 0 {
                current_template.0 = index + 2;
            }
            scope_count += 1;
        } else {
            scope_count -= 1;
            if scope_count == 0 {
                current_template.1 = index;
                templates.push(current_template);
            }
        }
    }
    if scope_count != 0 {
        log::debug!("{content}");
        log::error!("Opening/Closing tags mismatch.");
        return None;
    }
    Some(
        templates
            .iter()
            .map(|x| {
                let x = &content[x.0..x.1];
                return x;
            })
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_tags() {
        let test = "{{hello}} {{world}} {{outer {{inner}}}}";
        let tags = get_tags(test).unwrap();
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0], "hello");
        assert_eq!(tags[1], "world");
        assert_eq!(tags[2], "outer {{inner}}");

        let test = "{{hello}} {{there";
        let tags = get_tags(test);
        assert!(tags.is_none());
    }
}
