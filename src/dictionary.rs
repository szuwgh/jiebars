use crate::error::JResult;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Dictionary {
    dict: HashMap<String, u32>,
}

impl Dictionary {
    pub fn load(filepath: &Path) -> JResult<Dictionary> {
        let f = OpenOptions::new().read(true).open(filepath)?;
        let lines = BufReader::new(f).lines();
        Ok(Dictionary {
            dict: HashMap::new(),
        })
    }

    pub fn add_word() {}
}
