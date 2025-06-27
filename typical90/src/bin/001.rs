use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n],
    }

    let check = |x: usize| -> bool {
        let mut cnt = 0;
        let mut prev = 0;

        for &pos in &a {
            if pos - prev >= x {
                cnt += 1;
                prev = pos;
            }
        }

        if l - prev >= x {
            cnt += 1;
        }

        cnt >= k + 1
    };

    let mut left = 0;
    let mut right = l + 1;
    
    while right - left > 1 {
        let mid = (left + right) / 2;
        if check(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}