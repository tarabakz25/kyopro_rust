use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut a = vec![0i64; n+1];

    for i in 0..m {
        input! {
            l: usize,
            r: usize,
        }

        a[l - 1] += 1;
        a[r] -= 1;
    }

    for i in 1..n+1 {
        a[i] += a[i - 1];
    }

    let ans = *a[..n].iter().min().unwrap();   

    println!("{}", ans);
}