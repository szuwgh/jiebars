use jiebars::Jieba;
use memory_stats::memory_stats;
use std::thread;
use std::time::Duration;
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

    if let Some(usage) = memory_stats() {
        println!("Current physical memory usage: {}", usage.physical_mem);
        println!("Current virtual memory usage: {}", usage.virtual_mem);
    } else {
        println!("Couldn't get the current memory usage :(");
    }
    thread::sleep(Duration::from_secs(5));
}
