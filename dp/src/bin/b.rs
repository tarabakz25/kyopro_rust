use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
  input! {
    n: usize,
    k: usize,
    a: [usize; n],
  }

  let inf = 1 << 60;
  let mut dp = vec![inf; n];
  dp[0] = 0;

  for i in 1..n {
    dp[i] = dp[i-1] + (a[i-1] as i64 - a[i] as i64).abs() as usize;
    for j in 2..=k {
      if i >= j {
        dp[i] = dp[i].min(dp[i-j] + (a[i-j] as i64 - a[i] as i64).abs() as usize);
      }
    }
  }
  println!("{}", dp[n-1]);
}