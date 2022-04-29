use lazy_static::lazy_static;
use regex::{Match, Matches, Regex};

lazy_static! {
    pub static ref RE_HAN_DEFAULT: Regex =
        Regex::new(r"([\u4E00-\u9FD5a-zA-Z0-9+#&\._%\-]+)").unwrap();
    pub static ref RE_SKIP_DEAFULT: Regex = Regex::new(r"(\r\n|\s)").unwrap();
    pub static ref RE_SKIP_CUT_ALL: Regex = Regex::new(r"[^[:alnum:]+#\n]").unwrap();
}

pub struct SegmentMatches<'r, 't> {
    matches: Matches<'r, 't>,
    text: &'t str,
    last: usize,
    matched: Option<Match<'t>>,
}

impl<'r, 't> SegmentMatches<'r, 't> {
    pub fn new(re: &'r Regex, text: &'t str) -> SegmentMatches<'r, 't> {
        SegmentMatches {
            matches: re.find_iter(text),
            text: text,
            last: 0,
            matched: None,
        }
    }
}

pub enum SegmentState<'t> {
    Unmatched(&'t str),
    Matched(Match<'t>),
}

impl<'t> SegmentState<'t> {
    pub fn into_str(self) -> &'t str {
        match self {
            SegmentState::Unmatched(s) => s,
            SegmentState::Matched(m) => m.as_str(),
        }
    }
}

impl<'r, 't> Iterator for SegmentMatches<'r, 't> {
    type Item = SegmentState<'t>;

    fn next(&mut self) -> Option<SegmentState<'t>> {
        if let Some(m) = self.matched.take() {
            return Some(SegmentState::Matched(m));
        }
        match self.matches.next() {
            None => {
                if self.last == self.text.len() {
                    None
                } else {
                    let s = &self.text[self.last..];
                    self.last = self.text.len();
                    Some(SegmentState::Unmatched(s))
                }
            }
            Some(m) => {
                if self.last == m.start() {
                    self.last = m.end();
                    Some(SegmentState::Matched(m))
                } else {
                    let um = &self.text[self.last..m.start()];
                    self.last = m.end();
                    self.matched = Some(m);
                    Some(SegmentState::Unmatched(um))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_seg_chinese_text() {
        let seg = SegmentMatches::new(
            &RE_HAN_DEFAULT,
            "👪 PS: I have two match the, 我觉得开源有一个好处，就是能够敦促自己不断改进 👪，避免敞帚自珍",
        );
        for state in seg {
            match state {
                SegmentState::Matched(m) => {
                    println!("Matched:{:?}", m.as_str());
                }
                SegmentState::Unmatched(s) => {
                    println!("Unmatched:{:?}", s);
                }
            }
        }
    }

    #[test]
    fn test_re_skip_cut_all() {
        let seg = SegmentMatches::new(&RE_SKIP_CUT_ALL, "I have two match the");
        for state in seg {
            match state {
                SegmentState::Matched(m) => {
                    println!("Matched:{:?}", m.as_str());
                }
                SegmentState::Unmatched(s) => {
                    println!("Unmatched:{:?}", s);
                }
            }
        }
    }
}
