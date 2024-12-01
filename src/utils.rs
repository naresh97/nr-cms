pub trait StringChecks {
    fn is_newline(&self) -> bool;
}
impl StringChecks for str {
    fn is_newline(&self) -> bool {
        matches!(self, "\r\n" | "\r" | "\n")
    }
}

pub trait FoldStr {
    fn fold(&self) -> String;
}
impl FoldStr for [&str] {
    fn fold(&self) -> String {
        self.iter().fold(String::new(), |mut a, b| {
            a += *b;
            a
        })
    }
}
