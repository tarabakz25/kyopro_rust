use proconio::input;

fn main() {
  input! {
    n: usize,
    h: usize,
    m: usize,
    ab: [(usize, usize), n],
  }

  let mut dp = vec![None; h+1];
  dp[h] = m;

  for (i, &(a, b)) in ab.iter().enumerate() {
    let mut n_dp = vec![None; h+1];

    for (h, opt_mp) in dp.iter().enumerate() {
      if let
    }
  }

  dp = n_dp;
  if max(dp) < 0 {
    println!("{}", i - 1);
  }
}