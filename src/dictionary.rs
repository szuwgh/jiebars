use crate::error::JResult;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub struct Dictionary {
    dict: HashMap<String, u32>,
}

impl Dictionary {
    pub fn load(filepath: &Path) -> JResult<Dictionary> {
        let f = OpenOptions::new().read(true).open(filepath)?;
        let lines = BufReader::new(f).lines();
        let mut db = Dictionary {
            dict: HashMap::new(),
        };
        db.add_word(lines);
        Ok(db)
    }

    pub fn add_word(&mut self, lines: Lines<BufReader<File>>) {
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
                let cs = elem[0].chars();
                for c in cs {
                    self.dict.entry(c.to_string()).or_insert(0);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split_chinese_str() {
        let s = "程序设计艺术";
        let cs = s.chars();
        for c in cs {
            println!("{}", c);
        }
    }
}
