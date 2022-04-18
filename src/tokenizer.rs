use crate::dictionary::Dictionary;
use crate::error::JResult;
use crate::segment;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::{self, Chars};

const DEFAULT_WORD_LEN: usize = 32;

pub struct Tokenizer {
    dict: Dictionary,
}

type Route = (f64, usize);

// 基于前缀词典实现高效的词图扫描，生成句子中汉字所有可能成词情况所构成的有向无环图 (DAG)
// 采用了动态规划查找最大概率路径, 找出基于词频的最大切分组合
// 对于未登录词，采用了基于汉字成词能力的 HMM 模型，使用了 Viterbi 算法
impl Tokenizer {
    pub fn new() -> JResult<Tokenizer> {
        let dict = Dictionary::load("dict.txt")?;
        Ok(Tokenizer { dict })
    }

    //精确模式
    pub fn calc(&self, sentence: &str) -> Vec<Route> {
        let str_len = sentence.len();
        let dag = self.dag(sentence);
        let mut rs: Vec<Route> = vec![(0f64, 0); str_len + 1];
        let byte_index = sentence.char_indices().map(|x| x.0).rev();
        let mut r = (0.0, 0);
        let mut prev_byte_start = str_len;
        for byte_start in byte_index {
            let l = dag.get(&byte_start).unwrap();
            let pair = l
                .iter()
                .map(|byte_end| {
                    if let Some(freq) = self.dict.frequency(&sentence[byte_start..*byte_end]) {
                        r.0 = freq.ln() - self.dict.log_total + rs[*byte_end].0;
                    } else {
                        r.0 = 1f64.ln() - self.dict.log_total + rs[*byte_end].0;
                    }
                    r.1 = *byte_end;
                    r
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

    pub fn cut_dag_no_hmm<'a>(&self, text: &'a str) -> Vec<&'a str> {
        let mut words: Vec<&str> = Vec::with_capacity(DEFAULT_WORD_LEN);
        //正则分词 切成英语短语和汉字短语
        let segs = segment::seg_chinese_text(text);
        for sentence in segs.into_iter() {
            if sentence.trim() == "" {
                continue;
            }
            let rs = self.calc(sentence);
            let mut x = 0usize;
            let mut left: Option<usize> = None;
            while x < sentence.len() {
                let y = rs[x].1;
                let frag = &sentence[x..y];
                if frag.chars().count() == 1 && frag.chars().all(|ch| ch.is_ascii_alphanumeric()) {
                    if left.is_none() {
                        Some(x);
                        x = y;
                    }
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
        words
    }

    //
    pub fn cut_all<'a>(&self, text: &'a str) -> Vec<&'a str> {
        let mut words: Vec<&str> = Vec::with_capacity(DEFAULT_WORD_LEN);
        //正则分词 切成英语短语和汉字短语
        let segs = segment::seg_chinese_text(text);
        for seg in segs.into_iter() {
            if seg.trim() == "" {
                continue;
            }
            self.cut_allw(seg, &mut words)
        }
        words
    }

    fn cut_allw<'a>(&self, sentence: &'a str, words: &mut Vec<&'a str>) {
        //   let cs = sentence.chars();
        let dag = self.dag(sentence);
        let mut start: i32 = -1;
        let byte_index: Vec<usize> = sentence.char_indices().map(|x| x.0).collect();
        for (i, byte_start) in byte_index.into_iter().enumerate() {
            let l = dag.get(&byte_start).unwrap();
            for j in l {
                words.push(&sentence[byte_start..*j]);
            }
        }
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
            if tmplist.len() == 0 {
                tmplist.push(k);
            }
            dag.insert(k, tmplist);
        }
        dag
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dag() {
        let tokenizer = Tokenizer::new().unwrap();
        let dag = tokenizer.dag("我来到北京清华大学");
        print!("dag:{:?}", dag);
    }

    #[test]
    fn test_cut_all() {
        let tokenizer = Tokenizer::new().unwrap();
        let words = tokenizer.cut_all("我来到北京清华大学");
        print!("words:{:?}", words.join("/"));
    }

    #[test]
    fn test_cut_c() {
        let tokenizer = Tokenizer::new().unwrap();
        let rs = tokenizer.calc("我来到北京清华大学");
        print!("rs:{:?}", rs);
    }

    #[test]
    fn test_cut_dag_no_hmm() {
        let tokenizer = Tokenizer::new().unwrap();
        let words = tokenizer.cut_dag_no_hmm("我来到北京清华大学");
        print!("rs:{:?}", words);
    }
}
