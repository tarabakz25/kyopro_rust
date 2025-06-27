use proconio::input;

fn main() {
  input! {
    n: usize,
    h: usize,
    m: usize,
    a: [(usize, usize), n],
  }

  let mut dp = vec![-1; h+1];
  dp[h] = m;

  for i in 0..n {
    let mut n_dp = vec![-1; h+1];

    for j in 0..h {
      let m = dp[h];
      if m < 0 { continue; }

      if h >= a[i][0] {
        let h2 = h - a[i][0];
        n_dp[h2] = n_dp[h2].max(m);
      }
      if m >= a[i][0] {
        let m2 = m - a[i][0];
        n_dp[h2] = n_dp[h2].max(m);
      }
    }
  }

  dp = n_dp;
  if max(dp) < 0 {
    println!("{}", i - 1);
  }
}