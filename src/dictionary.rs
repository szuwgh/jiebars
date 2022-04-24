use crate::error::JResult;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

//把数据文件读进内存
static DEFAULT_DICT: &str = include_str!("data/dict.txt");

pub struct Dictionary {
    dict: HashMap<String, f64>,
    total: f64,
    pub log_total: f64,
}

impl Dictionary {
    pub fn load() -> JResult<Dictionary> {
        let lines = BufReader::new(DEFAULT_DICT.as_bytes()).lines();
        let mut db = Dictionary {
            dict: HashMap::new(),
            total: 0f64,
            log_total: 0f64,
        };
        db.add_word(lines);
        db.log_total = db.total.ln();
        Ok(db)
    }

    pub fn add_word(&mut self, lines: Lines<BufReader<&[u8]>>) {
        for res_line in lines {
            if let Ok(line) = res_line {
                let elem = line.split_whitespace().collect::<Vec<&str>>();
                if elem.len() != 3 {
                    continue;
                }
                let u = match elem[1].parse::<f64>() {
                    Ok(u) => u,
                    Err(_e) => continue,
                };
                self.total += u;
                self.dict.insert(elem[0].to_string(), u);
                let cs = elem[0].chars().collect::<Vec<char>>();
                for i in 0..cs.len() {
                    self.dict
                        .entry(cs[..i + 1].iter().collect())
                        .or_insert(0f64);
                }
            }
        }
    }

    pub fn frequency(&self, key: &str) -> Option<f64> {
        self.dict.get(key).map(|x| *x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::os::unix::prelude::FileExt;
    use std::str::Chars;
    #[test]
    fn test_split_chinese_str() {
        let s = "程序设计艺术";
        let cs = s.chars();
        let ss = cs.collect::<Vec<char>>();
        println!("{:?}", ss);
    }

    #[test]
    fn test_dictionary() {
        let dict = Dictionary::load().unwrap();
        if let Some(freq) = dict.frequency("我们") {
            println!("freq:{}", freq);
        }
    }
}
