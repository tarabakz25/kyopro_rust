use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
  input! {
    mut n: usize,
  }

  let mut ans = String::new();
  for t in (0..3).rev() {
    let d = 36u32.pow(t as u32);
    let i = 1;
    loop {
      if n < d as usize * i {
        if i - 1 > 9 {
          let t_chars = t.chars().collect();
          if t_chars[i] == s_chars[i] && i > 0 {
            println!("{}", t_chars[i]);
          }
          ans.push((b'a' + (i - 1) as u8) as char);
        }
        else {
          ans.push((i - 1) as char);
        }
      }
      i += 1;
    }
  }

  println!("{}", ans);
}
