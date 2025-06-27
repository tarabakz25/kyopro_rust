use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut arr = Vec::new();

  for i in 0..n {
    arr.push((a[i], i));
  }

  arr.sort_by(|x, y| x.0.cmp(&y.0).reverse());

  let mut ans = vec![0; n];

  for i in 0..n {
    ans[arr[i].1] = i+1;
    if i > 0 && arr[i].0 == arr[i-1].0 {
      ans[arr[i].1] = ans[arr[i-1].1];
    }
  }

  for i in 0..n {
    println!("{}", ans[i]);
  }
}