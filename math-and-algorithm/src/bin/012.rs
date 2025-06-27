use proconio::input;

fn main() {
    input! { n: usize,}

    if isPrime(n) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn isPrime(n: usize) -> bool {
    if n < 2 {
        return false;
    }

    for i in (2..=n).take_while(|i| i * i <= n) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}