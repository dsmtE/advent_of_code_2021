use regex::{CaptureMatches, Captures, Regex};

fn extract_from_match<'t, const N: usize>(match_captures: Captures<'t>) -> (&'t str, [&'t str; N]) {
    let mut captures = match_captures.iter().flatten();
    let whole_match = captures.next().unwrap().as_str();
    let captured = [0; N].map(|_| captures.next().unwrap().as_str());
    assert!(
        captures.next().is_none(),
        "too many participating capture groups"
    );
    (whole_match, captured)
}

pub struct RegexExtractIter<'r, 't, const N: usize> {
    captures: CaptureMatches<'r, 't>,
}

impl<'r, 't, const N: usize> Iterator for RegexExtractIter<'r, 't, N> {
    type Item = (&'t str, [&'t str; N]);

    fn next(&mut self) -> Option<Self::Item> {
        self.captures.next().map(extract_from_match)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.captures.size_hint()
    }
}
pub trait RegexExtract {
    /// Finds the leftmost-first match in `text` and returns a tuple containing the whole match
    /// and its N participating capture groups as strings. If no match is found, `None` is returned.
    ///
    /// Panics if the number of participating captures is not equal to N.
    fn extract<'t, const N: usize>(&self, text: &'t str) -> Option<(&'t str, [&'t str; N])>;

    fn extract_iter<'r, 't, const N: usize>(&'r self, text: &'t str) -> RegexExtractIter<'r, 't, N>;
}

impl RegexExtract for Regex {
    fn extract<'t, const N: usize>(&self, text: &'t str) -> Option<(&'t str, [&'t str; N])> {
        self.captures(text).map(extract_from_match)
    }

    fn extract_iter<'r, 't, const N: usize>(&'r self, text: &'t str) -> RegexExtractIter<'r, 't, N> {
        RegexExtractIter { captures: self.captures_iter(text) }
    }
}