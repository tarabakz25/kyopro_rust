use proconio::input;

fn main() {
  input! {
    n: usize, 
  }

  let mut ans = String::new();

  if n % 2 == 0 {
    ans.push_str(&"-".repeat((n - 2) / 2));
    ans.push_str("==");
    ans.push_str(&"-".repeat((n - 2) / 2));
  } else {
    ans.push_str(&"-".repeat((n - 1) / 2));
    ans.push_str("=");
    ans.push_str(&"-".repeat((n - 1) / 2));
  }

  println!("{}", ans);
} 