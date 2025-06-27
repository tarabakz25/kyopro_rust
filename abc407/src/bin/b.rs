use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    }

    let mut cnt = 0;
    for i in 1..7 {
        for j in 1..7 {
            if i + j >= x || (i - j).abs() >= y {
                cnt += 1;
            }
        }
    }

    let ans: f64 = cnt as f64 / 36.0;

    println!("{}", ans);
}
