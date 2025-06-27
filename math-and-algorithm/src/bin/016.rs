use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! { n: usize, a: [usize; n], }

    let mut ans = a[0];
    for i in 1..n {
        ans = gcd(ans, a[i]);
    }

    println!("{}", ans);
}