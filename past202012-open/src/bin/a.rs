use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main(){
  input! {
    s: String,
  }

  let s_chars: Vec<char> = s.chars().collect();
  for i in 1..s_chars.len()-1 {
    if s_chars[i] == s_chars[i-1] && s_chars[i] == s_chars[i+1] {
      if s_chars[i] == 'x' {
        println!("x");
        return;
      } else if s_chars[i] == 'o' {
        println!("o");
        return;
      }
    }
  }

  println!("draw");
}
