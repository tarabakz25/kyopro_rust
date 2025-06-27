use proconio::input;

fn main() {
    input! { n: usize, a: [u64; n], }

    let mut arr = vec![0u64; 4];

    for i in 0..n {
        if a[i] == 100 {
            arr[0] += 1;
        } else if a[i] == 200 {
            arr[1] += 1;
        } else if a[i] == 300 {
            arr[2] += 1;
        } else {
            arr[3] += 1;
        }
    }

    let ans: u64 = arr[0] * arr[3] + arr[1] * arr[2];
    println!("{}", ans);
}