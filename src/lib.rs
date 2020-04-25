pub trait Delimiter {
    // Option<(start, end)>
    fn find_next(&self, slice: &str) -> Option<(usize, usize)>;
}

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        StrSplit {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        // "let" keyword is also a pattern matching like "if let Some(x)"
        // "?" operator returns something inside Option<T> or None
        let reminder = self.remainder.as_mut()?;

        if let Some((delim_start, delim_end)) = self.delimiter.find_next(reminder) {
            let prefix = &reminder[..delim_start];
            *reminder = &reminder[delim_end..];
            Some(prefix)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    // Option<(start, end)>
    fn find_next(&self, slice: &str) -> Option<(usize, usize)> {
        slice.find(self).map(|idx| (idx, idx + self.len()))
    }
}

impl Delimiter for char {
    // Option<(start, end)>
    fn find_next(&self, slice: &str) -> Option<(usize, usize)> {
        slice
            .char_indices()
            .find(|(_idx, ch)| ch == self)
            .map(|(idx, _ch)| (idx, idx + self.len_utf8()))
    }
}

#[allow(dead_code)]
pub fn prefix_until_char(s: &str, ch: char) -> &str {
    StrSplit::new(s, ch)
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
