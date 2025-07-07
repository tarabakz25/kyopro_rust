use proconio::{fastout, input, marker::Chars};
use std::time::{Duration, Instant};

// ---------- 軽量 PRNG (xoroshiro128+) ----------
#[derive(Clone)] struct Rng(u64, u64);
impl Rng {
    #[inline] fn new(seed: u64) -> Self { Self(seed, seed ^ 0x9e3779b97f4a7c15) }
    #[inline] fn next(&mut self) -> u64 {
        let s0 = self.0; let mut s1 = self.1;
        let r = s0.wrapping_add(s1);
        s1 ^= s0;
        self.0 = s0.rotate_left(55) ^ s1 ^ (s1 << 14);
        self.1 = s1.rotate_left(36);
        r
    }
    #[inline] fn f64(&mut self) -> f64 { (self.next() >> 11) as f64 * (1.0/9.007e15) }
    #[inline] fn usize(&mut self, m: usize) -> usize { (self.next() % m as u64) as usize }
}

// ---------- 40×40 ビットボード ----------
#[derive(Clone)]
struct Board { row: [u64; 40], col: [u64; 40] }
impl Board {
    #[inline] fn make_rock(&mut self, x: usize, y: usize) {
        self.row[x] |= 1u64 << y; self.col[y] |= 1u64 << x;
    }
    #[inline]
    fn slide(&self, x: usize, y: usize, dx: i8, dy: i8) -> (usize, usize) {
        if dx != 0 {
            let col = self.col[y];
            if dx > 0 {
                // ↓（下）方向
                let below = col >> (x + 1);
                let step = below.trailing_zeros() as usize + 1;   // かならずヒット
                (x + step, y)
            } else {
                // ↑（上）方向
                let mask = if x == 0 { 0 } else { col & ((1u64 << x) - 1) };
                if mask == 0 {
                    (0, y)                         // 外周まで氷：最上段で停止
                } else {
                    let nz = 63 - mask.leading_zeros() as usize;
                    (nz, y)
                }
            }
        } else {
            let row = self.row[x];
            if dy > 0 {
                // →（右）方向
                let right = row >> (y + 1);
                let step = right.trailing_zeros() as usize + 1;
                (x, y + step)
            } else {
                // ←（左）方向
                let mask = if y == 0 { 0 } else { row & ((1u64 << y) - 1) };
                if mask == 0 {
                    (x, 0)                         // 最左端で停止
                } else {
                    let nz = 63 - mask.leading_zeros() as usize;
                    (x, nz)
                }
            }
        }
    }
}

// ---------- ゲーム完全再現 ----------
fn score(order: &[(usize, usize)]) -> i64 {
    let mut bd = Board { row: [u64::MAX; 40], col: [u64::MAX; 40] };
    for x in 1..39 { bd.row[x] = (1u64 << 40) - 1; }      // 内側を岩→あとで穴あけ
    for &(x, y) in order {
        bd.row[x] &= !(1u64 << y); bd.col[y] &= !(1u64 << x);
    }

    let mut pos = order[0];
    bd.make_rock(pos.0, pos.1);
    const DIR: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut sc = 0i64; let mut alpha = 1i64;
    for &(tx, ty) in &order[1..] {
        // 到達可能方向＝最短距離優先
        let mut ok = None;
        for &(dx, dy) in &DIR {
            if bd.slide(pos.0, pos.1, dx, dy) == (tx, ty) { ok = Some((dx, dy)); break; }
        }
        if ok.is_none() { break; }
        let (dx, dy) = ok.unwrap();

        // 滑走路を岩化
        let (mut x, mut y) = (pos.0 as i32, pos.1 as i32);
        loop {
            bd.make_rock(x as usize, y as usize);
            if (x as usize, y as usize) == (tx, ty) { break; }
            x += dx as i32; y += dy as i32;
        }
        sc += alpha; alpha += 1;
        pos = (tx, ty);
    }
    sc
}

// ---------- 焼きなまし ----------
fn annealing(mut ord: Vec<(usize, usize)>, limit: Duration) -> Vec<(usize, usize)> {
    let st = Instant::now();
    let mut rng = Rng::new(1234567);
    let mut best = ord.clone(); let mut bestv = score(&best) as f64;
    let mut curv = bestv; let mut t = 500.0;

    while st.elapsed() < limit {
        // 近傍（2-opt or cut）
        match rng.usize(2) {
            0 => {           // 2-opt
                let i = rng.usize(ord.len() - 2) + 1;
                let j = rng.usize(ord.len() - 2) + 1;
                if i < j { ord[i..=j].reverse(); } else { ord[j..=i].reverse(); }
            }
            1 => {           // cut & paste
                let l = rng.usize(ord.len() - 6) + 1;
                let r = l + rng.usize(5) + 1;          // 長さ 1〜5
                let seg: Vec<_> = ord.drain(l..r).collect();
                let k = rng.usize(ord.len() - 1) + 1;
                ord.splice(k..k, seg);
            }
            _ => unreachable!(),
        }
        let nv = score(&ord) as f64;
        let diff = nv - curv;
        if diff > 0.0 || rng.f64() < (diff / t).exp() {
            curv = nv;
            if nv > bestv { bestv = nv; best = ord.clone(); }
        } else {
            ord.clone_from(&best); curv = bestv;       // rollback
        }
        t = (t * 0.9995).max(0.01);
    }
    best
}

// ---------- main ----------
#[fastout]
fn main() {
    input! { n: usize, m: usize, field: [Chars; n] }

    // 氷マス収集
    let mut ice = Vec::<(usize, usize)>::with_capacity(n * n - m);
    for i in 0..n {
        for j in 0..n {
            if field[i][j] == '.' { ice.push((i, j)); }
        }
    }

    // スネーク初期列
    ice.sort_by_key(|&(x, y)| (y / 2 * 40) + if y % 2 == 0 { x } else { 39 - x });

    let best = annealing(ice, Duration::from_millis(1_900));
    for (x, y) in best { println!("{x} {y}"); }
}
