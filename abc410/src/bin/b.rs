use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }

    let mut balls = vec![0; n]; // n個のボールの値を管理
    let mut result = Vec::new();

    for i in 0..q {
        if x[i] != 0 {
            // x[i]番目のボール（1-indexed）を取る
            balls[x[i] - 1] += 1;
            result.push(x[i]);
        } else {
            // 現在の値が最小のボールを取る（複数ある場合は番号が最小）
            let mut min_idx = 0;
            for j in 1..n {
                if balls[j] < balls[min_idx] {
                    min_idx = j;
                }
            }
            balls[min_idx] += 1;
            result.push(min_idx + 1); // 1-indexedで出力
        }
    }

    // 結果を空白区切りで出力
    let ans = result.iter()
        .map(|&x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", ans);
}