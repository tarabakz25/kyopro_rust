use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut q_two = Vec::new();
    let mut q_one = Vec::new();

    let mut prev = 0;

    for i in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                c: usize,
                x: usize,
            }

            q_one.push((c, x));
        }
        else if t == 2 {
            input! {
                mut k: usize,
            }

            q_two.push(k + prev);
            prev += k;
        }
    }

    let mut arr = Vec::new();
    let mut prev = 0;
    let mut cnt = 0;
    let mut idx = 0;
    
    for i in 0..q_one.len() {
        let (c, x) = q_one[i];

        for j in 0..c {
            arr.push(x + prev);
            prev += x;

            if arr.len() == q_two[idx] {
                println!("{}", arr[idx] - cnt);
                idx += 1;
                cnt += arr[idx - 1];
            }
        }
    }
}




