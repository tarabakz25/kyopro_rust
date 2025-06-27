use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut a_sorted = a.clone();

    a_sorted.sort();
    a_sorted.dedup();

    println!("{}", a_sorted.len());

    for i in 0..a_sorted.len() {
        println!("{}", a_sorted[i]);
    }
}