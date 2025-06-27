use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        t: [usize; n],
    }


    for i in 0..n {
        if i == 0 {
            if t[i] > s {
                println!("No");
                return;
            }
        } else {
            if t[i] - t[i - 1] > s {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}