use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut activities = Vec::new();
  for _ in 0..n {
    input! {
      a: usize,
      b: usize,
      c: usize,
    }

    activities.push([a, b, c]);
  }

  let mut dp = vec![vec![0; 3]; n];
  
  dp[0][0] = activities[0][0];
  dp[0][1] = activities[0][1];
  dp[0][2] = activities[0][2];

  for i in 1..n {
    dp[i][0] = dp[i-1][1].max(dp[i-1][2]) + activities[i][0];
    dp[i][1] = dp[i-1][0].max(dp[i-1][2]) + activities[i][1];
    dp[i][2] = dp[i-1][0].max(dp[i-1][1]) + activities[i][2];
  }

  println!("{}", dp[n-1][0].max(dp[n-1][1].max(dp[n-1][2])));
}