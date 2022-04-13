use regex::Regex;

//一个汉字 3 个字节
pub fn seg_chinese_text<'a>(text: &'a str) -> Vec<&'a str> {
    let r: Regex = Regex::new(r"([\u4e00-\u9fa5a-zA-Z0-9+#&._%-])+").unwrap();
    let t = r.find_iter(text);
    let mut s: Vec<&str> = Vec::new();
    let mut begin: usize = 0;
    let mut end: usize = 0;
    for m in t {
        println!("{:?}", m);
        s.push(&text[m.start()..m.end()]);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_seg_chinese_text() {
        let v = seg_chinese_text("I have two num，用来帮助用户与操作系统进行沟通。");
        println!("{:?}", v);
    }
}
