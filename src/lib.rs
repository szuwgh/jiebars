mod dictionary;
mod error;
mod hmm;
mod hmm_data;
mod segment;

use crate::dictionary::Dictionary;
use crate::error::JResult;
use crate::segment::{SegmentMatches, SegmentState, RE_HAN_DEFAULT, RE_SKIP_DEAFULT};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::{self, Chars};

const DEFAULT_WORD_LEN: usize = 32;

pub struct Jieba {
    dict: Dictionary,
}

type Route = (f64, usize);

// Based on the front-end implementation of efficient word graph scanning of the dictionary, a directed acyclic graph (DAG) composed of all possible word formations in the sentence is generated
// Use the maximum range segmentation combination based on dynamic search terms
// For unregistered words, the HMM model based on the ability of Chinese characters to form words is used, and the Viterbi algorithm is used
impl Jieba {
    pub fn new() -> JResult<Jieba> {
        let dict = Dictionary::load()?;
        Ok(Jieba { dict })
    }

    //获取有向无环图
    fn dag(&self, sentence: &str) -> HashMap<usize, Vec<usize>> {
        let mut dag: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut frag: &str;
        let mut n = sentence.len();
        let mut i: usize = 0;
        for (k, _) in sentence.char_indices().peekable() {
            let mut tmplist: Vec<usize> = Vec::new();
            let mut remain = sentence[k..].char_indices().peekable();
            loop {
                if let Some((j, _)) = remain.next() {
                    if j == 0 {
                        continue;
                    }
                    i = k + j;
                } else {
                    i = n;
                }
                frag = &sentence[k..i];
                if let Some(f) = self.dict.frequency(frag) {
                    if f > 0f64 {
                        tmplist.push(i);
                    }
                }
                if i == n {
                    break;
                }
            }
            dag.insert(k, tmplist);
        }
        dag
    }

    //精确模式
    fn calc(&self, sentence: &str) -> Vec<Route> {
        let str_len = sentence.len();
        let dag = self.dag(sentence);
        let mut rs: Vec<Route> = vec![(0f64, 0); str_len + 1];
        let byte_index = sentence.char_indices().map(|x| x.0).rev();
        let mut prev_byte_start = str_len;
        for byte_start in byte_index {
            let l = dag.get(&byte_start).unwrap();
            let pair = l
                .iter()
                .map(|byte_end| {
                    let mut f: f64 = 0.0;
                    if let Some(freq) = self.dict.frequency(&sentence[byte_start..*byte_end]) {
                        f = freq.ln() - self.dict.log_total + rs[*byte_end].0;
                    } else {
                        f = 1f64.ln() - self.dict.log_total + rs[*byte_end].0;
                    }
                    (f, *byte_end)
                })
                .max_by(|r1, r2| r1.partial_cmp(r2).unwrap_or(Ordering::Equal));

            if let Some(p) = pair {
                rs[byte_start] = p;
            } else {
                let byte_end = prev_byte_start;
                rs[byte_start] = (1f64.ln() - self.dict.log_total + rs[byte_end].0, byte_end);
            }
            prev_byte_start = byte_start;
        }
        rs
    }

    fn cut_dag_with_hmm<'a>(&self, sentence: &'a str, words: &mut Vec<&'a str>) {
        let rs = self.calc(sentence);
        let mut x = 0usize;
        let mut left: Option<usize> = None;
        while x < sentence.len() {
            let y = rs[x].1;
            let frag = &sentence[x..y];
            if frag.chars().count() == 1 {
                if left.is_none() {
                    left = Some(x);
                }
            } else {
                if let Some(l) = left {
                    let word = &sentence[l..x];
                    if word.chars().count() == 1 {
                        words.push(word);
                    } else {
                        let f = self.dict.frequency(word);
                        if f.is_none() || f == Some(0.0) {
                            hmm::cut(word, words);
                        } else {
                            let mut word_index = word.char_indices().map(|x| x.0).peekable();
                            while let Some(byte_start) = word_index.next() {
                                let byte_end = *word_index.peek().unwrap_or(&word.len());
                                words.push(&word[byte_start..byte_end]);
                            }
                        }
                    }
                    left = None;
                }
                words.push(frag);
            }
            x = y;
        }
        if let Some(l) = left {
            let word = &sentence[l..];
            if word.chars().count() == 1 {
                words.push(word);
            } else {
                let f = self.dict.frequency(word);
                if f.is_none() || f == Some(0.0) {
                    hmm::cut(word, words);
                } else {
                    let mut word_index = word.char_indices().map(|x| x.0).peekable();
                    while let Some(byte_start) = word_index.next() {
                        let byte_end = *word_index.peek().unwrap_or(&word.len());
                        words.push(&word[byte_start..byte_end]);
                    }
                }
            }
        }
    }

    fn cut_dag_no_hmm<'a>(&self, sentence: &'a str, words: &mut Vec<&'a str>) {
        let rs = self.calc(sentence);
        let mut x = 0usize;
        let mut left: Option<usize> = None;
        while x < sentence.len() {
            let y = rs[x].1;
            let frag = &sentence[x..y];
            if frag.chars().count() == 1 && frag.chars().all(|ch| ch.is_ascii_alphanumeric()) {
                if left.is_none() {
                    left = Some(x);
                }
                x = y;
                continue;
            }
            if let Some(l) = left {
                words.push(&sentence[l..y]);
                left = None;
            }
            words.push(frag);
            x = y;
        }
        if let Some(l) = left {
            words.push(&sentence[l..]);
        }
    }

    fn cut_all<'a>(&self, sentence: &'a str, words: &mut Vec<&'a str>) {
        let dag = self.dag(sentence);
        //let start: i32 = -1;
        let byte_index: Vec<usize> = sentence.char_indices().map(|x| x.0).collect();
        for (i, byte_start) in byte_index.into_iter().enumerate() {
            let l = dag.get(&byte_start).unwrap();
            for j in l {
                words.push(&sentence[byte_start..*j]);
            }
        }
    }

    pub fn cut<'a>(&self, text: &'a str, cut_all: bool, hmm: bool) -> Vec<&'a str> {
        let mut words: Vec<&str> = Vec::with_capacity(DEFAULT_WORD_LEN);
        let seg_split = SegmentMatches::new(&RE_HAN_DEFAULT, text);
        for m in seg_split {
            match m {
                SegmentState::Matched(m) => {
                    if cut_all {
                        self.cut_all(m.as_str(), &mut words)
                    } else if hmm {
                        self.cut_dag_with_hmm(m.as_str(), &mut words)
                    } else {
                        self.cut_dag_no_hmm(m.as_str(), &mut words)
                    }
                }
                SegmentState::Unmatched(s) => {}
            }
        }
        words
    }

    pub fn cut_for_search<'a>(&self, text: &'a str) -> Vec<&'a str> {
        let words = self.cut(text, false, true);
        let mut new_words = Vec::with_capacity(words.len());
        for word in words.iter() {
            let char_len = word.chars().count();
            let char_index: Vec<usize> = word.char_indices().map(|x| x.0).collect();
            for v in 2..=3 {
                if char_len <= v {
                    continue;
                }
                for i in 0..char_len - v {
                    let garm = &word[char_index[i]..char_index[i + v]];
                    if let Some(f) = self.dict.frequency(garm) {
                        if f > 0.0 {
                            new_words.push(garm);
                        }
                    }
                }
            }
            new_words.push(word);
        }
        new_words
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dag() {
        let jieba = Jieba::new().unwrap();
        let dag = jieba.dag("two");
        print!("dag:{:?}", dag);
    }

    #[test]
    fn test_cut_all() {
        let jieba = Jieba::new().unwrap();
        let words = jieba.cut("小明硕士毕业于中国科学院计算所", true, false);
        print!("rs:{:?}", words);
    }

    #[test]
    fn test_cut_c() {
        let jieba = Jieba::new().unwrap();
        let rs = jieba.calc("two");
        print!("rs:{:?}", rs);
    }

    #[test]
    fn test_cut_dag_no_hmm() {
        let jieba = Jieba::new().unwrap();
        let words = jieba.cut("我来到北京清华大学", false, false);
        print!("rs:{:?}", words);
    }

    #[test]
    fn test_cut_dag_hmm() {
        let jieba = Jieba::new().unwrap();
        let words = jieba.cut("I have two ok小明硕士毕业于中国科学院计算所", false, true);
        print!("rs:{:?}", words);
    }

    #[test]
    fn test_cut_for_search() {
        let jieba = Jieba::new().unwrap();
        let words = jieba.cut_for_search("小明硕士毕业于中国科学院计算所");
        print!("rs:{:?}", words);
    }
}
