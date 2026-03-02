/// Join args and format the Obsidian append content string.
pub fn build_content(timestamp: &str, args: &[String]) -> String {
    format!("- {} | {}", timestamp, args.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_word() {
        assert_eq!(build_content("2026-03-02 14:05", &["hello".into()]), "- 2026-03-02 14:05 | hello");
    }

    #[test]
    fn multiple_words_joined_with_space() {
        let args = ["foo", "bar", "baz"].map(String::from).to_vec();
        assert_eq!(build_content("2026-03-02 14:05", &args), "- 2026-03-02 14:05 | foo bar baz");
    }

    #[test]
    fn preserves_special_characters() {
        let args = ["hello, world!"].map(String::from).to_vec();
        assert_eq!(build_content("T", &args), "- T | hello, world!");
    }
}
