use proconio::input;

fn main() {
    input! { n: usize,}

    let mut arr = Vec::new();

    let mut i = 1;
    while i <= n / i {
        if n % i == 0 {
            arr.push(i);
            if i != n / i {
                arr.push(n / i);
            }
        }
        i += 1;
    }

    arr.sort();

    for i in arr {
        println!("{}", i);
    }
}