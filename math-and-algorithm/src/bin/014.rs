use proconio::input;

fn main() {
    input! { mut n: usize, }

    let mut ans = Vec::new();
    let mut ans_str = String::new();

    loop {
        let mut i = 2;
        let mut isPrime = true;

        while i * i <= n {
            if n % i == 0 {
                ans.push(i);
                n /= i;
                isPrime = false;
                break;
            }
            i += 1;
        }

        if isPrime {
            ans.push(n);
            break;
        }
    }

    ans.sort();

    for i in ans {
        ans_str.push_str(&i.to_string());
        ans_str.push_str(" ");
    }

    println!("{}", ans_str.trim());
}