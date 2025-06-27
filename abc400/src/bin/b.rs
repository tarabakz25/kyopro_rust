use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut cnt = 0;

    for i in 0..m+1 {
        cnt += n.pow(i as u32);

        if cnt > 10_usize.pow(9) {
            println!("inf");
            return;
        }
    }

    println!("{}", cnt);
}