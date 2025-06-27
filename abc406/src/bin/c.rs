use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize, n],
    }

    let mut a_points = Vec::new();
    let mut b_points = Vec::new();

    for i in 1..n-1 {
        if p[i] > p[i-1] && p[i] > p[i+1] {  
            a_points.push(i);
        }
        if p[i] < p[i-1] && p[i] < p[i+1] {
            b_points.push(i);
        }
    }

}