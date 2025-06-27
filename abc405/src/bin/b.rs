use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }
    let mut arr = vec![false; m + 1];
    let mut cnt = 0;

    for i in 0..n {
        if arr[a[i]] == false {
            cnt += 1;
            arr[a[i]] = true;
        }

        if cnt == m {
            println!("{}", n - i);
            return;
        }
    }

    println!("{}", 0);

}