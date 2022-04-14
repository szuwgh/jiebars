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

    pub fn cut_all(text: &str) {
        //正则分词 切成英语短语和汉字短语
        let segs = segment::seg_chinese_text(text);
        for seg in segs.into_iter() {
            if seg.trim() == "" {
                continue;
            }
        }
    }

    fn cut_allw(&self, sentence: &str) {
        let words: Vec<&str> = Vec::with_capacity(DEFAULT_WORD_LEN);
        let cs = sentence.chars();
        let dag = self.dag(cs);
        let start: i32 = -1;
        for (k, l) in dag.into_iter().enumerate() {
            if l.len() == 1 && k as i32 > start {}
        }
    }

    //获取有向无环图
    fn dag(&self, cs: Chars) -> Vec<Vec<u32>> {
        let mut dag: Vec<Vec<u32>> = Vec::new();
        let sentence = cs.collect::<Vec<char>>();
        let mut i: usize = 0;
        let n = sentence.len();
        let mut frag: &[char];
        for k in 0..n {
            let mut tmplist: Vec<u32> = Vec::new();
            i = k;
            frag = &sentence[k..k + 1];
            while i < n {
                if let Some(f) = self.dict.frequency(frag.iter().collect()) {
                    if *f > 0 {
                        tmplist.push(i as u32);
                    }
                    i += 1;
                    if i >= n {
                        break;
                    }
                    frag = &sentence[k..i + 1];
                } else {
                    break;
                }
            }
            if tmplist.len() == 0 {
                tmplist.push(k as u32);
            }
            dag.push(tmplist)
            //dag.insert(k as u32, tmplist);
        }
        dag
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::os::unix::prelude::FileExt;
    use std::str::Chars;
    #[test]
    fn test_dag() {
        let tokenizer = Tokenizer::new().unwrap();
        let dag = tokenizer.dag("我来到北京清华大学".chars());
        print!("dag:{:?}", dag);
    }
}
