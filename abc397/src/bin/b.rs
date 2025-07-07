use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        s: String,
    }

    let mut ans = 0;
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut s_len = s_chars.len();

    let mut i = 0;

    while i < s_len {
        if i % 2 == 0 {
            if s_chars[i] != 'i' {
                ans += 1;
                s_chars.insert(i, 'i');
                s_len += 1;
                continue;
            }
        }
        else {
            if s_chars[i] != 'o' {
                ans += 1;
                s_chars.insert(i, 'o');
                s_len += 1;
                continue;
            }
        }
        i += 1;
    }

    if s_len % 2 == 1 {
        ans += 1;
    }

    println!("{}", ans);
}
