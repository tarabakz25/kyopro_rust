// src/bin/d.rs
use proconio::input;

fn is_square(n: i128) -> bool {
    if n < 0 {
        return false;
    }
    let root = (n as f64).sqrt() as i128;
    root * root == n || (root + 1) * (root + 1) == n
}

fn main() {
    input! {
        n: u128,
    }

    for i in 1..=((n as f64).sqrt() as u128) {
        if n % i != 0 {
            continue;
        }
        let d = i;
        let m = n / d;
        // S = -3d^2 + 12m
        let d_i128 = d as i128;
        let m_i128 = m as i128;
        let s = -3 * d_i128 * d_i128 + 12 * m_i128;
        if s < 0 || !is_square(s) {
            continue;
        }
        let sqrt_s = (s as f64).sqrt() as i128;
        // y = (-3d + sqrt_s) / 6
        if (-3 * d_i128 + sqrt_s) % 6 != 0 {
            continue;
        }
        let y = (-3 * d_i128 + sqrt_s) / 6;
        if y <= 0 {
            continue;
        }
        let x = y + d_i128;
        println!("{} {}", x, y);
        return;
    }
    println!("-1");
}
