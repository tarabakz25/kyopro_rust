use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut check_num = vec![0; n + 1];
    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();

    let mut cnt = 0;
    for i in 0..n {
        if check_num[a[i]] == 0 {
            check_num[a[i]] = 1;
            cnt += 1;
        }
        arr1.push(cnt);
    }

    check_num = vec![0; n + 1];
    cnt = 0;
    
    for i in (0..n).rev() {
        if check_num[a[i]] == 0 {
            check_num[a[i]] = 1;
            cnt += 1;
        }
        arr2.push(cnt);
    }

    arr2.reverse();

    let mut ans = 0;
    for i in 0..n {
        if ans  < arr1[i] + arr2[i] - 1 {
            ans = arr1[i] + arr2[i] - 1;
        }
    }

    println!("{}", ans);
}

