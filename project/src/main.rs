use proconio::input;
use rand::Rng;
use std::time::Instant;

const N: usize = 20;
const OPS_LIMIT: usize = 2 * N * N * N;
const TIME_LIMIT_MS: u128 = 1900;
const FAIL_LIMIT: usize = 200;

#[derive(Clone)]
struct BoxState {
    w: u64,
    rem: i64,
}

#[inline]
fn manhattan(a: (usize, usize), b: (usize, usize)) -> u64 {
    ((a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()) as u64
}

fn move_and_update(stack: &mut Vec<BoxState>, dist: u64) -> bool {
    if dist == 0 || stack.is_empty() {
        return true;
    }
    let total_w: u64 = stack.iter().map(|b| b.w).sum();
    let mut weight_above = total_w;
    for b in stack.iter_mut() {
        weight_above -= b.w;
        b.rem -= (weight_above * dist) as i64;
        if b.rem <= 0 {
            return false;
        }
    }
    true
}

fn simulate_move(stack: &Vec<BoxState>, dist: u64) -> bool {
    if dist == 0 || stack.is_empty() {
        return true;
    }
    let mut tmp = stack.clone();
    move_and_update(&mut tmp, dist)
}

fn main() {
    input! {
        _n: usize,
        w_in: [[u64; N]; N],
        d_in: [[u64; N]; N],
    }

    let start = Instant::now();
    let mut rng = rand::thread_rng();

    let mut remaining: Vec<(usize, usize)> = (0..N)
        .flat_map(|i| (0..N).map(move |j| (i, j)))
        .filter(|&(i, j)| !(i == 0 && j == 0))
        .collect();

    let mut pos = (0usize, 0usize);
    let mut stack: Vec<BoxState> = Vec::new();
    let mut ops: usize = 0;

    while !remaining.is_empty()
        && ops + 10 < OPS_LIMIT
        && start.elapsed().as_millis() < TIME_LIMIT_MS
    {
        let mut fail = 0;

        while fail < FAIL_LIMIT
            && !remaining.is_empty()
            && ops + 10 < OPS_LIMIT
            && start.elapsed().as_millis() < TIME_LIMIT_MS
        {
            let idx = rng.gen_range(0..remaining.len());
            let (y, x) = remaining[idx];

            let dist_to = manhattan(pos, (y, x));
            if !simulate_move(&stack, dist_to) {
                fail += 1;
                continue;
            }

            let mut tmp_stack = stack.clone();
            move_and_update(&mut tmp_stack, dist_to);
            tmp_stack.push(BoxState {
                w: w_in[y][x],
                rem: d_in[y][x] as i64,
            });
            let dist_back = (y + x) as u64;
            if !simulate_move(&tmp_stack, dist_back) {
                fail += 1;
                continue;
            }

            let vertical = if y > pos.0 { ('D', y - pos.0) } else { ('U', pos.0 - y) };
            for _ in 0..vertical.1 {
                println!("{}", vertical.0);
            }
            let horizontal = if x > pos.1 { ('R', x - pos.1) } else { ('L', pos.1 - x) };
            for _ in 0..horizontal.1 {
                println!("{}", horizontal.0);
            }
            ops += dist_to as usize;
            move_and_update(&mut stack, dist_to);

            println!("1");
            ops += 1;
            stack.push(BoxState {
                w: w_in[y][x],
                rem: d_in[y][x] as i64,
            });

            pos = (y, x);
            remaining.swap_remove(idx);
            fail = 0;
        }

        if !stack.is_empty() {
            let (y, x) = pos;
            for _ in 0..y {
                println!("U");
            }
            for _ in 0..x {
                println!("L");
            }
            ops += (y + x) as usize;
            move_and_update(&mut stack, (y + x) as u64);

            stack.clear();
            pos = (0, 0);
        }
    }

    for (y, x) in remaining {
        for _ in 0..y {
            println!("D");
        }
        for _ in 0..x {
            println!("R");
        }
        println!("1");
        for _ in 0..y {
            println!("U");
        }
        for _ in 0..x {
            println!("L");
        }
        println!("2");
    }
}
