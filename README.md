# jiebars

[jieba](https://github.com/fxsjy/jieba)分词rust版实现

特点
========
* 支持4种模式
    * 全模式：把句子中所有的可以成词的词语都扫描出来, 速度非常快，但是不能解决歧义
    * 精确模式：试图将句子最精确地切开，适合文本分析
    * 新词识别模式：对于未登录词，采用了基于汉字成词能力的 HMM 模型，使用了 Viterbi 算法
    * 搜索引擎模式：在精确模式的基础上，对长词再次切分，提高召回率，适合用于搜索引擎分词

代码示例
```rust
use jiebars::Jieba;

fn main() {
    let jieba = Jieba::new().unwrap();

    //全模式
    let mut words = jieba.cut("我来到北京清华大学", true, false);

    println!("\n【全模式】:{}\n", words.join(" / "));

    //精确模式
    words = jieba.cut("他来到了网易杭研大厦", false, false);

    println!("【精确模式】:{}\n", words.join(" / "));

    //新词识别模式
    words = jieba.cut("他来到了网易杭研大厦", false, true);

    println!("【新词识别模式】:{}\n", words.join(" / "));

    //搜索引擎模式
    words = jieba.cut_for_search("小明硕士毕业于中国科学院计算所，后在日本京都大学深造");

    println!("【搜索引擎模式】:{}\n", words.join(" / "));
}
```

输出:

【全模式】:我 / 来 / 来到 / 到 / 北 / 北京 / 京 / 清 / 清华 / 清华大学 / 华 / 华大 / 大 / 大学 / 学

【精确模式】:他 / 来到 / 了 / 网易 / 杭 / 研 / 大厦

【新词识别模式】:他 / 来到 / 了 / 网易 / 杭研 / 大厦

【搜索引擎模式】:小明 / 硕士 / 毕业 / 于 / 中国 / 科学 / 中国科学院 / 计算 / 计算所 / 后 / 后在 / 日本 / 京都 / 日本京都大学 / 深造