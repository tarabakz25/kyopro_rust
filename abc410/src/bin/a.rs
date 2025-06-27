use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
  input! {
    n: usize,
    a: [usize; n],
    k: usize,
  }

  let mut ans = 0;

  for i in 0..n {
    if a[i] >= k {
      ans += 1;
    }
  }

  println!("{}", ans);
}