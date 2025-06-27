use proconio::input;

fn main() {
    input! { mut n: usize, mut m: usize, } 

    let mut r = n % m;
    n = m;
    m = r;

    loop {
        r = n % m;
        if r == 0 {
            break;
        }
        n = m;
        m = r;
    }

    println!("{}", m);
}