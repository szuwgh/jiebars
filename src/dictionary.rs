use crate::error::JResult;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

//把数据文件读进内存
static DEFAULT_DICT: &str = include_str!("data/dict.txt");

pub struct Dictionary {
    dict: HashMap<String, u32>,
}

impl Dictionary {
    pub fn load(filepath: &str) -> JResult<Dictionary> {
        //let f = OpenOptions::new().read(true).open(filepath)?;
        let lines = BufReader::new(DEFAULT_DICT.as_bytes()).lines();
        let mut db = Dictionary {
            dict: HashMap::new(),
        };
        db.add_word(lines);
        Ok(db)
    }

    pub fn add_word(&mut self, lines: Lines<BufReader<&[u8]>>) {
        for res_line in lines {
            if let Ok(line) = res_line {
                let elem = line.split_whitespace().collect::<Vec<&str>>();
                if elem.len() != 3 {
                    continue;
                }
                let u = match elem[1].parse::<u32>() {
                    Ok(u) => u,
                    Err(_e) => continue,
                };
                self.dict.insert(elem[0].to_string(), u);
                let cs = elem[0].chars().collect::<Vec<char>>();
                for i in 0..cs.len() {
                    self.dict.entry(cs[..i + 1].iter().collect()).or_insert(0);
                }
            }
        }
    }

    pub fn frequency(&self, key: String) -> Option<&u32> {
        self.dict.get(&key)
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
        // for c in cs {
        //     println!("{}", c);
        // }
        let ss = cs.collect::<Vec<char>>();
        println!("{:?}", ss);
    }

    #[test]
    fn test_dictionary() {
        // opt/rsproject/jiebars/dict.txt
        // let base_dir = env::current_dir().expect("not found path");
        // let file_dir = &base_dir.join("dict.txt");
        let dict = Dictionary::load(DEFAULT_DICT).unwrap();
        if let Some(freq) = dict.frequency("我们".to_string()) {
            println!("freq:{}", *freq);
        }
        // println!("The base dir: {}", base_dir.to_str().expect(""));
    }
}
