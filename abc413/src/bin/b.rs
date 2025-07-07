use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        a: [String; n],
    }

    let mut arr = Vec::new();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue; // Skip if both indices are the same
            }
            let s1 = &a[i];
            let s2 = &a[j];
            let s = format!("{}{}", s1, s2);
            arr.push(s);
        }
    }

    arr.sort();
    arr.dedup();

    println!("{}", arr.len());
}
