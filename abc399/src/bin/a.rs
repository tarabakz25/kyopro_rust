use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        s: String,
        t: String,
    }

    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    let mut ans = 0;
    for i in 0..n {
        if s_chars[i] != t_chars[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}