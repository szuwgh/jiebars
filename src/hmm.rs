use crate::hmm_data::{PROB_EMITS, PROB_START, PROB_TRANS};
use crate::segment::SegmentMatches;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::str::Chars;

lazy_static! {
    static ref RE_HAN: Regex = Regex::new(r"([\u{4E00}-\u{9FD5}]+)").unwrap();
    static ref RE_SKIP: Regex = Regex::new(r"([a-zA-Z0-9]+(?:.\d+)?%?)").unwrap();
}

const MIN_FLOAT: f64 = -3.14e100;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum Status {
    B = 0,
    E = 1,
    M = 2,
    S = 3,
}

static PREV_STATUS: [[Status; 2]; 4] = [
    [Status::E, Status::S], // B
    [Status::B, Status::M], // E
    [Status::M, Status::B], // M
    [Status::S, Status::E], // S
];

pub(crate) fn cut<'a>(sentence: &'a str, words: &mut Vec<&'a str>) {
    if RE_HAN.is_match(sentence) {
        if sentence.chars().count() > 1 {
            cut_han(sentence, words);
        } else {
            words.push(sentence);
        }
    } else {
        let skip_splitter = SegmentMatches::new(&RE_SKIP, sentence);
        for skip_state in skip_splitter {
            let x = skip_state.into_str();
            if x.is_empty() {
                continue;
            }
            words.push(x);
        }
    }
}

fn cut_han<'a>(sentence: &'a str, words: &mut Vec<&'a str>) {
    let str_len = sentence.len();
    let pos_list = viterbi(sentence);
    let mut curr = sentence.char_indices().map(|x| x.0).peekable();
    let mut begin: usize = 0;
    let mut next: usize = 0;
    let mut i: usize = 0;
    while let Some(byte_start) = curr.next() {
        let pos = pos_list[i];
        match pos {
            Status::B => begin = byte_start,
            Status::E => {
                let byte_end = *curr.peek().unwrap_or(&str_len);
                words.push(&sentence[begin..byte_end]);
                next = byte_end;
            }
            Status::M => {}
            Status::S => {
                let byte_end = *curr.peek().unwrap_or(&str_len);
                words.push(&sentence[begin..byte_end]);
                next = byte_end;
            }
        }
        i += 1;
    }
    if next < str_len {
        words.push(&sentence[next..]);
    }
}

// obs : 观察值集合
// (B, M, E, S) : 状态值集合
// InitStatus :初始状态概率分布
// 转移概率矩阵Status(i)只和Status(i-1)相关
// 发射概率矩阵: P(Observed[i], Status[j]) = P(Status[j]) * P(Observed[i]|Status[j])
fn viterbi(obs: &str) -> Vec<Status> {
    let str_len = obs.len();
    let status = [Status::B, Status::M, Status::E, Status::S];

    let R = status.len();
    let C = obs.chars().count();
    let mut V: Vec<f64> = vec![0f64; R * C];

    let mut path: Vec<Status> = vec![Status::B; C];
    let mut prev: Vec<Option<Status>> = vec![None; R * C];

    let mut curr = obs.char_indices().map(|x| x.0).peekable();
    let x1 = curr.next().unwrap();
    let x2 = *curr.peek().unwrap();
    for y in &status {
        let first_word = &obs[x1..x2];
        let _y = *y as usize;
        let prob = PROB_START[_y] + PROB_EMITS[_y].get(first_word).cloned().unwrap_or(MIN_FLOAT);
        V[_y] = prob;
    }

    let mut t = 1;
    while let Some(byte_start) = curr.next() {
        let byte_end = *curr.peek().unwrap_or(&str_len);
        let word = &obs[byte_start..byte_end];
        for y in &status {
            let _y = *y as usize;
            let em_prob = PROB_EMITS[_y].get(word).cloned().unwrap_or(MIN_FLOAT);
            let (prob, state) = PREV_STATUS[_y]
                .iter()
                .map(|y0| {
                    let _y0 = *y0 as usize;
                    (
                        V[(t - 1) * R + _y0]
                            + PROB_TRANS[_y0].get(_y).cloned().unwrap_or(MIN_FLOAT)
                            + em_prob,
                        *y0,
                    )
                })
                .max_by(|x, y| x.partial_cmp(y).unwrap_or(Ordering::Equal))
                .unwrap();
            let idx = (t * R) + (*y as usize);
            V[idx] = prob;
            prev[idx] = Some(state);
        }
        t += 1;
    }
    //最后一个字的状态只可能是 E 或者 S，不可能是 M 或者 B, 只需要比较 weight[1(E)][14] 和 weight[3(S)][14] 的大小
    let (_prob, state) = [Status::E, Status::S]
        .iter()
        .map(|y| (V[(C - 1) * R + (*y as usize)], y))
        .max_by(|x, y| x.partial_cmp(y).unwrap_or(Ordering::Equal))
        .unwrap();
    let mut t = C - 1;
    let mut curr = *state;
    path[t] = *state;

    //回溯的路径
    while let Some(p) = prev[t * R + (curr as usize)] {
        assert!(t > 0);
        path[t - 1] = p;
        curr = p;
        t -= 1;
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viterbi() {
        let path = viterbi("小明硕士毕业于中国科学院计算所");
        println!("path:{:?}", path);
    }

    #[test]
    fn test_cut_han() {
        let mut words: Vec<&str> = Vec::with_capacity(64);
        cut_han("two", &mut words);
        println!("words:{:?}", words);
    }
}
