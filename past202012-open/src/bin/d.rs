use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
  input! {
    n: usize,
    mut a: [String; n],
  }

  a.sort_by(|x, y| {
    let ta = x.trim_start_matches('0');
    let tb = y.trim_start_matches('0');

   
  })
}
