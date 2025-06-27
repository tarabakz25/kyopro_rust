use proconio::input;

fn main() {
    input! { n: usize, mut a: [u64; n], }

    a.sort();

    let mut ans: u64 = 0;
    let mut cnt = [0u64; 4];

    for i in 0..n {
        cnt[a[i] as usize] += 1;
    }

    for i in 1..4 {
        ans += cnt[i] * (cnt[i] - 1) / 2;
    }

    println!("{}", ans);
}