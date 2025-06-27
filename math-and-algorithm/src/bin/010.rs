use proconio::input;

fn main() {
    input! { n: usize, }

    let mut ans = n;

    for i in (1..n).rev() {
        ans *= i;
    }

    println!("{}", ans);
}