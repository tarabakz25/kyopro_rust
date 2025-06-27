use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let inf = 1 << 60;  
  let mut dp = vec![inf; n];
  dp[0] = 0;

  for i in 1..n {
    dp[i] = dp[i].min(dp[i-1] + (a[i] as i64 - a[i-1] as i64).abs() as usize);
    if i >= 2 {
      dp[i] = dp[i].min(dp[i-2] + (a[i] as i64 - a[i-2] as i64).abs() as usize);
    }
  }

  println!("{}", dp[n-1]);
}