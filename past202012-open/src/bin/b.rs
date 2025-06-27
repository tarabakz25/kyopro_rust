use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
  input! {
     n: usize,
     s: String,
  }

  let s_chars: Vec<char> = s.chars().collect();
  let mut t: Vec<char> = Vec::new();

  for i in 0..s_chars.len() {
    t.retain(|&x| x != s_chars[i]);

    t.push(s_chars[i]);
  }

  let ans: String = t.into_iter().collect();
  println!("{}", ans);
}
