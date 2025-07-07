use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(usize, usize); n],
    }

    let mut ans = 0;

    for (x, y) in a {
        if x < y {
            ans += 1;
        }
    }

    println!("{}", ans);
}
