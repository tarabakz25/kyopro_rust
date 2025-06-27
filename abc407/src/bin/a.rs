use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let ans = (a / b).round() as usize;

    println!("{}", ans);

}