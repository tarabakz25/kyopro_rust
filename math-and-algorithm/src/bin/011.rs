use proconio::input;

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

fn main() {
    input! { n: usize, } 

    for i in 0..=n {
        if isPrime(i) {
            println!("{}", i);
        }
    }
}