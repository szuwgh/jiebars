use jiebars::Jieba;

fn main() {
    let jieba = Jieba::new().unwrap();

    //全模式
    let mut words = jieba.cut("我来到北京清华大学", true, false);

    println!("\n【全模式】:{}\n", words.join("/"));

    //精确模式
    words = jieba.cut("我来到北京清华大学", false, true);

    println!("【精确模式】:{}\n", words.join("/"));

    //搜索引擎模式
    words = jieba.cut_for_search("小明硕士毕业于中国科学院计算所，后在日本京都大学深造");

    println!("【搜索引擎模式】:{}\n", words.join("/"));
}
