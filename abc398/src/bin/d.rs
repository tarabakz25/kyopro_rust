use proconio::input;

fn main() {
  input! {
    n: usize,
    r: isize,
    c: isize,
    s: String,
  }

  let s_chars = s.chars().collect::<Vec<char>>();
  let mut ans = String::new();

    let t = s_chars[i];
    if t == 'N' {
      takahashi_idx.1 += 1;
      takibi_idx.1 += 1;
      field[takibi_idx.0 as usize][takibi_idx.1 as usize] = true;
    }
    else if t == 'S' {
      takahashi_idx.1 -= 1;
      takibi_idx.1 -= 1;
      field[takibi_idx.0 as usize][takibi_idx.1 as usize] = true;
    }
    else if t == 'E' {
      takahashi_idx.0 += 1;
      takibi_idx.0 += 1;
      field[takibi_idx.0 as usize][takibi_idx.1 as usize] = true;
    }
    else if t == 'W' {
      takahashi_idx.0 -= 1;
      takibi_idx.0 -= 1;
      field[takibi_idx.0 as usize][takibi_idx.1 as usize] = true;
    }

    if field[takahashi_idx.0 as usize][takahashi_idx.1 as usize] {
      ans.push('1');
    } else {
      ans.push('0');
    }
  }

  println!("{}", ans);
}
