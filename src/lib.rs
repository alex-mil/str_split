//!
// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimeter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimeter str,
}

impl<'haystack, 'delimeter> StrSplit<'haystack, 'delimeter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimeter str) -> Self {
        StrSplit {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack> Iterator for StrSplit<'haystack, '_> {
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        // "let" keyword is also a pattern matching like "if let Some(x)"
        // "?" operator returns something inside Option<T> or None
        let reminder = self.remainder.as_mut()?;

        if let Some(idx) = reminder.find(self.delimiter) {
            let prefix = &reminder[..idx];
            *reminder = &reminder[(idx + self.delimiter.len())..];
            Some(prefix)
        } else {
            self.remainder.take()
        }
    }
}

#[allow(dead_code)]
pub fn prefix_until_char(s: &str, ch: char) -> &str {
    StrSplit::new(s, &ch.to_string())
        .next()
        .expect("StrSplit always gives at least one result")
}

#[cfg(test)]
mod tests {
    use super::{prefix_until_char, StrSplit};

    #[test]
    fn it_works() {
        let input = "a b c d e";
        let letters: Vec<_> = StrSplit::new(input, " ").collect();

        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn tail() {
        let input = "a b c d ";
        let letters: Vec<_> = StrSplit::new(input, " ").collect();

        assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
    }

    #[test]
    fn prefix_until_char_test() {
        assert_eq!(prefix_until_char("hello world", 'w'), "hello ");
    }
}
