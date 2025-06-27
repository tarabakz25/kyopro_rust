use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
  input! {
    n: usize,
    q: usize,
  }

  let mut arr = Vec::new();
  for i in 0..n {
    arr.push(i+1);
  }

  let mut amount = 0;

  for _ in 0..q {
    input! { que: usize, }
    if que == 1 {
      input! { p: usize, x: usize, }
      arr[(p+amount-1) % n] = x;
    } else if que == 2 {
      input! { p: usize, }
      println!("{}", arr[(p+amount-1) % n]);
    } else {
      input! { k: usize, }
      amount += k;
    }
  }
}