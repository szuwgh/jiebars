use crate::hmm_data::{PROB_EMIT, PROB_START, PROB_TRANS};
use std::str::Chars;

const MIN_FLOAT: f64 = -3.14e100;

pub enum Status {
    B = 0,
    E = 1,
    M = 2,
    S = 3,
}

//obs : 观察值集合
//(B, M, E, S) : 状态值集合
//InitStatus :初始状态概率分布
// 转移概率矩阵Status(i)只和Status(i-1)相关
//发射概率矩阵: P(Observed[i], Status[j]) = P(Status[j]) * P(Observed[i]|Status[j])
fn viterbi(obs: &str) {
    let str_len = obs.len();
    let status = [Status::B, Status::M, Status::E, Status::S];
    let mut path: Vec<Status> = Vec::new();
    let r = status.len();
    let c = obs.chars().count();
    let V: Vec<f64> = vec![0f64; r * c];
    let mut curr = obs.char_indices().map(|x| x.0).peekable();
    let x1 = curr.next().unwrap();
    let x2 = *curr.peek().unwrap();
    for y in &status {
        let first_word = &obs[x1..x2];
        let _y = *y as usize;
        let prob = PROB_EMIT[_y].get(first_word).cloned().unwrap_or(MIN_FLOAT);
        V[_y] = prob;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viterbi() {
        let v = prob_emit[0].get("门");
        println!("v:{:?}", v);
    }
}
