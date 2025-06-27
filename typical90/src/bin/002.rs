use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let isvalid = |s: &str| -> bool {
        let mut score = 0;

        for c in s.chars() {
            if c == '(' {
                score += 1;
            } else {
                score -= 1;
            }

            if score < 0 {
                return false;
            }
        }

        score == 0
    };

    for i in 0..(1 << n) {
        let mut s = String::new();

        for j in (0..n).rev() {
            if i & (1 << j) == 0 {
                s.push('(');
            } else {
                s.push(')');
            }
        }

        if isvalid(&s) {
            println!("{}", s);
        }
    }
}