use regex::Regex;

//一个汉字 3 个字节
pub fn seg_chinese_text<'a>(text: &'a str) -> Vec<&'a str> {
    let r: Regex = Regex::new(r"(\p{Han}+)").unwrap();
    let t = r.find_iter(text);
    let mut s: Vec<&str> = Vec::new();
    let mut begin: usize = 0;
    for m in t {
        if begin != m.start() {
            s.push(&text[begin..m.start()]);
        }
        begin = m.end();
        s.push(&text[m.start()..m.end()]);
    }  
    if begin != text.len() {
        s.push(&text[begin..]);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test_seg_chinese_text() {
        let v = seg_chinese_text(
            "I have two num程序编程two num, ok he艺术，用来帮助用户与操作系统进行沟通。wwwwwww ssssss",
        );
        println!("{:?}", v);
    }
}
