use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut arr = Vec::new();
  for i in 0..n {
    arr.push((a[i], i + 1));
  }

  arr.sort_by(|a, b| a.0.cmp(&b.0));

  let mut ans = 0;

  for i in 0..n {
    let mut check = true;

    if i > 0 && arr[i].0 == arr[i - 1].0 {
      check = false;
    }
    if i < n - 1 && arr[i].0 == arr[i + 1].0 {
      check = false;
    }

    if check {
      ans = ans.max(arr[i].0);
    }
  }

  if ans == 0 {
    println!("-1");
  } else {
    for i in 0..n {
      if ans == arr[i].0 {
        println!("{}", arr[i].1);
        break;
      }
    }
  }
}
