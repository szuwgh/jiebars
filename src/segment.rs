use lazy_static::lazy_static;
use regex::{Match, Matches, Regex};

lazy_static! {
    static ref RE_HAN_DEFAULT: Regex = Regex::new(r"([\u4E00-\u9FD5a-zA-Z0-9+#&\._%\-]+)").unwrap();
    static ref RE_SKIP_DEAFULT: Regex = Regex::new(r"(\r\n|\s)").unwrap();
}

struct SegmentMatches<'r, 't> {
    matches: Matches<'r, 't>,
    text: &'t str,
    last: usize,
    matched: Option<Match<'t>>,
}

impl<'r, 't> SegmentMatches<'r, 't> {
    fn new(re: &'r Regex, text: &'t str) -> SegmentMatches<'r, 't> {
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

//一个汉字 3 个字节
// pub fn seg_chinese_text<'a>(text: &'a str) -> Vec<&'a str> {
//     let r: Regex = Regex::new(r"(\p{Han}+)").unwrap();
//     let t = r.find_iter(text);
//     let mut s: Vec<&str> = Vec::new();
//     let mut begin: usize = 0;
//     for m in t {
//         if begin != m.start() {
//             s.push(&text[begin..m.start()]);
//         }
//         begin = m.end();
//         s.push(&text[m.start()..m.end()]);
//     }
//     if begin != text.len() {
//         s.push(&text[begin..]);
//     }
//     s
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_seg_chinese_text() {
        let seg = SegmentMatches::new(
            &RE_HAN_DEFAULT,
            "👪 PS: I have two match the我觉得开源有一个好处，就是能够敦促自己不断改进 👪，避免敞帚自珍",
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
}
