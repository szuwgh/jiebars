use crate::dictionary::Dictionary;
use crate::error::JResult;
use crate::segment;
use std::collections::HashMap;
use std::str::Chars;

const DEFAULT_WORD_LEN: usize = 32;

pub struct Tokenizer {
    dict: Dictionary,
}

// 基于前缀词典实现高效的词图扫描，生成句子中汉字所有可能成词情况所构成的有向无环图 (DAG)
// 采用了动态规划查找最大概率路径, 找出基于词频的最大切分组合
// 对于未登录词，采用了基于汉字成词能力的 HMM 模型，使用了 Viterbi 算法
impl Tokenizer {
    pub fn new() -> JResult<Tokenizer> {
        let dict = Dictionary::load("dict.txt")?;
        Ok(Tokenizer { dict })
    }

    //精确模式
    pub fn cut_c<'a>(&self, text: &'a str) -> Vec<&'a str> {}

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
        let cs = sentence.chars();
        let dag = self.dag(sentence);
        let mut start: i32 = -1;
        let byte_index: Vec<usize> = sentence.char_indices().map(|x| x.0).collect();
        for (i, byte_start) in byte_index.into_iter().enumerate() {
            let l = &dag[i];
            for j in l {
                words.push(&sentence[byte_start..*j as usize]);
            }
        }
    }

    //获取有向无环图
    fn dag(&self, sentence: &str) -> Vec<Vec<u32>> {
        let mut dag: Vec<Vec<u32>> = Vec::new();
        let mut frag: &str;
        let mut n = sentence.len();
        let mut i: usize = 0;
        for (k, _) in sentence.char_indices().peekable() {
            let mut tmplist: Vec<u32> = Vec::new();
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
                    if f > 0 {
                        tmplist.push(i as u32);
                    }
                }
                if i == n {
                    break;
                }
            }
            if tmplist.len() == 0 {
                tmplist.push(k as u32);
            }
            dag.push(tmplist)
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
}
