use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut arr = Vec::new();
    let mut cnt = 0;

    for i in 0..n {
        cnt += a[i];
        arr.push(cnt);
    }

    let mut ans = 0;
    let mut sum = 0;
    for i in 0..n {
        ans += sum * a[i];
        sum += a[i];
    }

    println!("{}", ans);
}