use proconio::input;

fn main() {
    input! {
        a: [usize; 7],
    }

    let mut arr = vec![0; 13];
    for i in 0..7 {
      arr[a[i] - 1] += 1;
    }

    arr.sort_by(|a, b| b.cmp(a));

    if arr[0] >= 3 && arr[1] >= 2 {
      println!("Yes");
    } else {
      println!("No");
    }
} 