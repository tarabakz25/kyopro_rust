use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! { n: usize, mut a: [usize; n], }

    let mut ans = a[0];
    for i in 1..n {
        let g = gcd(ans, a[i]);
        ans = ans / g * a[i];
    }

    println!("{}", ans);
}