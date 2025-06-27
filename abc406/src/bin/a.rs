use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if a > c || (a == c && b > d) {
        println!("Yes");
    } else {
        println!("No");
    }
}