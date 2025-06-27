use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u128; n],
    }

    let mut ans: u128 = 1;
    let mut check_num = 1;
    for i in 0..k {
        check_num *= 10;
    }

    for i in 0..n {
        ans *= a[i];
        if ans >= check_num {
            ans = 1;
        }
    }
    println!("{}", ans);
}