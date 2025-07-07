use proconio::input;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::time::Instant;
use std::collections::HashMap;

// 定数
const L: i32 = 1000;
const M: f64 = 3000.0;
const RADIUS: i32 = 10;
const OUTER: i32 = RADIUS * 2;
const TIMELIMIT_MS: f64 = 15_000.0; // 焼きなまし実行時間[ms]

#[derive(Clone)]
struct Vortex {
    x: i32,
    y: i32,
    w: i32,
    candidates: Vec<(i32, i32)>, // 複数の通過点候補
}

// 距離キャッシュ
type DistCache = HashMap<((i32, i32), (i32, i32)), f64>;

fn dist(a: (i32, i32), b: (i32, i32)) -> f64 {
    let dx = (a.0 - b.0) as f64;
    let dy = (a.1 - b.1) as f64;
    (dx * dx + dy * dy).sqrt()
}

fn dist_cached(a: (i32, i32), b: (i32, i32), cache: &mut DistCache) -> f64 {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(&d) = cache.get(&key) {
        d
    } else {
        let d = dist(a, b);
        cache.insert(key, d);
        d
    }
}

/// 総行程長を計算
fn route_length(path: &[(i32, i32)]) -> f64 {
    path.windows(2).map(|p| dist(p[0], p[1])).sum()
}

/// 総行程長を計算（キャッシュ版）
fn route_length_cached(path: &[(i32, i32)], cache: &mut DistCache) -> f64 {
    path.windows(2).map(|p| dist_cached(p[0], p[1], cache)).sum()
}

/// 距離差分を返す
fn delta_len(path: &[(i32, i32)], idx: usize, insert: Option<(i32, i32)>) -> f64 {
    if idx + 1 >= path.len() { return 0.0; }
    let before = dist(path[idx], path[idx + 1]);
    let after = match insert {
        Some(pt) => dist(path[idx], pt) + dist(pt, path[idx + 1]),
        None => if idx + 2 < path.len() {
            dist(path[idx], path[idx + 2])
        } else {
            return 0.0; // 無効
        },
    };
    after - before
}

/// 距離差分を返す（キャッシュ版）
fn delta_len_cached(
    path: &[(i32, i32)], 
    idx: usize, 
    insert: Option<(i32, i32)>,
    cache: &mut DistCache
) -> f64 {
    if idx + 1 >= path.len() { return 0.0; }
    let before = dist_cached(path[idx], path[idx + 1], cache);
    let after = match insert {
        Some(pt) => {
            dist_cached(path[idx], pt, cache) + dist_cached(pt, path[idx + 1], cache)
        }
        None => {
            if idx + 2 < path.len() {
                dist_cached(path[idx], path[idx + 2], cache)
            } else {
                return 0.0;
            }
        },
    };
    after - before
}

/// 3-opt: 3つのエッジを削除して再接続
fn three_opt_delta(
    path: &[(i32, i32)], 
    i: usize, 
    j: usize, 
    k: usize,
    pattern: usize
) -> f64 {
    if i + 1 >= path.len() || j + 1 >= path.len() || k + 1 >= path.len() {
        return f64::INFINITY; // 範囲外は無効
    }
    // 削除される3つのエッジの長さ
    let removed = dist(path[i], path[i + 1]) 
        + dist(path[j], path[j + 1]) 
        + dist(path[k], path[k + 1]);
    
    // 追加される3つのエッジの長さ（パターンによって異なる）
    let added = match pattern {
        0 => dist(path[i], path[j]) + dist(path[i + 1], path[k]) + dist(path[j + 1], path[k + 1]),
        1 => dist(path[i], path[j]) + dist(path[i + 1], path[j + 1]) + dist(path[k], path[k + 1]),
        2 => dist(path[i], path[k]) + dist(path[j + 1], path[i + 1]) + dist(path[j], path[k + 1]),
        3 => dist(path[i], path[j + 1]) + dist(path[k], path[i + 1]) + dist(path[j], path[k + 1]),
        _ => return f64::INFINITY,
    };
    
    added - removed
}

/// 3-optのパターンに応じて経路を再構築
fn apply_three_opt(
    path: &mut Vec<(i32, i32)>,
    order: &mut Vec<usize>,
    i: usize,
    j: usize,
    k: usize,
    pattern: usize,
) {
    let n = path.len();
    let mut new_path = Vec::with_capacity(n);
    let mut new_order = Vec::with_capacity(order.len());
    
    // (0,0)を追加
    new_path.push(path[0]);
    
    match pattern {
        0 => {
            // [0,i] + [j,i+1] + [k,j+1] + [k+1,n-1]
            for idx in 1..=i {
                new_path.push(path[idx]);
                if idx <= order.len() {
                    new_order.push(order[idx - 1]);
                }
            }
            for idx in (i + 1..=j).rev() {
                new_path.push(path[idx]);
                if idx <= order.len() {
                    new_order.push(order[idx - 1]);
                }
            }
            for idx in (j + 1..=k).rev() {
                new_path.push(path[idx]);
                if idx <= order.len() {
                    new_order.push(order[idx - 1]);
                }
            }
            for idx in k + 1..n {
                new_path.push(path[idx]);
                if idx <= order.len() + 1 && idx > 1 {
                    new_order.push(order[idx - 2]);
                }
            }
        }
        1 => {
            // [0,i] + [j,i+1] + [j+1,k] + [k+1,n-1]
            for idx in 1..=i {
                new_path.push(path[idx]);
                if idx <= order.len() {
                    new_order.push(order[idx - 1]);
                }
            }
            for idx in (i + 1..=j).rev() {
                new_path.push(path[idx]);
                if idx <= order.len() {
                    new_order.push(order[idx - 1]);
                }
            }
            for idx in j + 1..=k {
                new_path.push(path[idx]);
                if idx <= order.len() {
                    new_order.push(order[idx - 1]);
                }
            }
            for idx in k + 1..n {
                new_path.push(path[idx]);
                if idx <= order.len() + 1 && idx > 1 {
                    new_order.push(order[idx - 2]);
                }
            }
        }
        2 => {
            // [0,i] + [k,j+1] + [i+1,j] + [k+1,n-1]
            for idx in 1..=i {
                new_path.push(path[idx]);
                if idx <= order.len() {
                    new_order.push(order[idx - 1]);
                }
            }
            for idx in (j + 1..=k).rev() {
                new_path.push(path[idx]);
                if idx <= order.len() {
                    new_order.push(order[idx - 1]);
                }
            }
            for idx in i + 1..=j {
                new_path.push(path[idx]);
                if idx <= order.len() {
                    new_order.push(order[idx - 1]);
                }
            }
            for idx in k + 1..n {
                new_path.push(path[idx]);
                if idx <= order.len() + 1 && idx > 1 {
                    new_order.push(order[idx - 2]);
                }
            }
        }
        3 => {
            // [0,i] + [j+1,k] + [j,i+1] + [k+1,n-1]
            for idx in 1..=i {
                new_path.push(path[idx]);
                if idx <= order.len() {
                    new_order.push(order[idx - 1]);
                }
            }
            for idx in j + 1..=k {
                new_path.push(path[idx]);
                if idx <= order.len() {
                    new_order.push(order[idx - 1]);
                }
            }
            for idx in (i + 1..=j).rev() {
                new_path.push(path[idx]);
                if idx <= order.len() {
                    new_order.push(order[idx - 1]);
                }
            }
            for idx in k + 1..n {
                new_path.push(path[idx]);
                if idx <= order.len() + 1 && idx > 1 {
                    new_order.push(order[idx - 2]);
                }
            }
        }
        _ => return,
    }
    
    *path = new_path;
    *order = new_order;
}

/// Or-opt: 連続するk個の要素を別の位置に移動
fn or_opt_delta(
    path: &[(i32, i32)],
    from: usize,
    to: usize,
    k: usize,
) -> Option<f64> {
    let n = path.len();
    if from + k >= n || to >= n - 1 {
        return None;
    }
    if to >= from && to < from + k {
        return None; // 無効な移動
    }
    
    // 削除されるエッジ
    let removed = dist(path[from], path[from + 1])
        + dist(path[from + k - 1], path[from + k])
        + if to < from {
            dist(path[to], path[to + 1])
        } else {
            dist(path[to - 1], path[to])
        };
    
    // 追加されるエッジ
    let added = dist(path[from], path[from + k])
        + if to < from {
            dist(path[to], path[from + 1])
                + dist(path[from + k - 1], path[to + 1])
        } else {
            dist(path[to - 1], path[from + 1])
                + dist(path[from + k - 1], path[to])
        };
    
    Some(added - removed)
}

/// Or-optを適用
fn apply_or_opt(
    path: &mut Vec<(i32, i32)>,
    order: &mut Vec<usize>,
    from: usize,
    to: usize,
    k: usize,
) {
    if from == 0 || from + k > path.len() - 1 {
        return; // 無効な操作
    }
    
    // 移動する部分を取り出す
    let mut segment_path: Vec<_> = path.drain(from..from + k).collect();
    let mut segment_order: Vec<_> = if from <= order.len() {
        order.drain(from - 1..((from - 1 + k).min(order.len()))).collect()
    } else {
        vec![]
    };
    
    // 新しい位置に挿入
    let insert_pos = if to < from { to } else { to - k };
    for (i, pt) in segment_path.into_iter().enumerate() {
        path.insert(insert_pos + i, pt);
    }
    
    if insert_pos > 0 && insert_pos <= order.len() + 1 {
        let order_insert_pos = (insert_pos - 1).min(order.len());
        for (i, ord) in segment_order.into_iter().enumerate() {
            order.insert(order_insert_pos + i, ord);
        }
    }
}

/// 最近挿入法による初期解生成
fn nearest_insertion(vortices: &[Vortex], cache: &mut DistCache) -> (Vec<usize>, Vec<(i32, i32)>, Vec<usize>, i32, f64) {
    let n = vortices.len();
    let mut path = vec![(0, 0), (L, L)];
    let mut order = vec![];
    let mut cand_idx = vec![0; n];
    let mut score = 1;
    let mut len = route_length_cached(&path, cache);
    let mut used = vec![false; n];
    
    // 未訪問の渦潮がある限り続ける
    while order.len() < n {
        let mut best_vortex = None;
        let mut best_pos = 0;
        let mut best_delta = f64::INFINITY;
        let mut best_cand = 0;
        
        // 各未訪問の渦潮について
        for (v_idx, v) in vortices.iter().enumerate() {
            if used[v_idx] {
                continue;
            }
            
            // 各候補点について
            for (c_idx, &cand) in v.candidates.iter().enumerate() {
                // 現在の経路のどこに挿入するのが最適か
                for pos in 0..path.len() - 1 {
                    let delta = delta_len_cached(&path, pos, Some(cand), cache);
                    if delta < best_delta && len + delta <= M {
                        best_vortex = Some(v_idx);
                        best_pos = pos;
                        best_delta = delta;
                        best_cand = c_idx;
                    }
                }
            }
        }
        
        // 最良の挿入を実行
        if let Some(v_idx) = best_vortex {
            let pt = vortices[v_idx].candidates[best_cand];
            path.insert(best_pos + 1, pt);
            order.insert(best_pos, v_idx);
            cand_idx[v_idx] = best_cand;
            used[v_idx] = true;
            score += vortices[v_idx].w;
            len += best_delta;
        } else {
            break; // これ以上挿入できない
        }
    }
    
    (order, path, cand_idx, score, len)
}

fn main() {
    input! {
        n: usize, _l: i32, _m: i32, _r: i32,
        p: [(i32, i32, i32); n],
    }
    
    // 距離キャッシュを初期化
    let mut dist_cache: DistCache = HashMap::new();
    
    // 32方向の候補点を生成
    const NUM_ANGLES: usize = 32;
    let vortices: Vec<Vortex> = p
        .into_iter()
        .map(|(x, y, w)| {
            let mut candidates = Vec::new();
            for i in 0..NUM_ANGLES {
                let angle = 2.0 * std::f64::consts::PI * i as f64 / NUM_ANGLES as f64;
                let px = x + (OUTER as f64 * angle.cos()).round() as i32;
                let py = y + (OUTER as f64 * angle.sin()).round() as i32;
                candidates.push((px, py));
            }
            Vortex { x, y, w, candidates }
        })
        .collect();

    // 複数の初期解を試す
    let mut best_init = (vec![], vec![], vec![], 0, f64::INFINITY);
    
    // 1. 最近挿入法
    let nearest_result = nearest_insertion(&vortices, &mut dist_cache);
    if nearest_result.3 > best_init.3 {
        best_init = nearest_result;
    }
    
    // 2. 既存の貪欲法（改良版）
    {
        let mut order: Vec<usize> = vec![];
        let mut path: Vec<(i32, i32)> = vec![(0, 0), (L, L)];
        let mut score = 1;
        let mut len = route_length(&path);
        let mut cand_idx: Vec<usize> = vec![0; n];

        let mut items: Vec<(f64, usize, usize)> = vec![];
        for (i, v) in vortices.iter().enumerate() {
            // 各渦潮の全候補点を評価
            for (j, &cand) in v.candidates.iter().enumerate() {
                let base = dist((0, 0), cand) + dist(cand, (L, L)) - len;
                if base > 0.0 {
                    items.push((-(v.w as f64 / base), i, j));
                }
            }
        }
        items.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        let mut used = vec![false; n];
        for (_, idx, cand_pt_idx) in items {
            if used[idx] {
                continue;
            }
            let pt = vortices[idx].candidates[cand_pt_idx];
            let mut pos = 0usize;
            let mut best_delta = f64::INFINITY;
            for k in 0..path.len() - 1 {
                let d = delta_len(&path, k, Some(pt));
                if d < best_delta {
                    best_delta = d;
                    pos = k;
                }
            }
            if len + best_delta <= M {
                len += best_delta;
                path.insert(pos + 1, pt);
                order.insert(pos, idx);
                cand_idx[idx] = cand_pt_idx;
                used[idx] = true;
                score += vortices[idx].w;
            }
        }
        
        if score > best_init.3 {
            best_init = (order, path, cand_idx, score, len);
        }
    }
    
    // 最良の初期解を使用
    let (mut best_order, mut best_path, mut best_cand_idx, mut best_score, mut best_len) = best_init;

    // ③ SA
    let start = Instant::now();
    let timelimit = TIMELIMIT_MS;
    let mut rng = thread_rng();
    let mut order = best_order.clone();
    let mut path = best_path.clone();
    let mut cand_idx = best_cand_idx.clone();
    let mut score = best_score;
    let mut len = best_len;
    let t0 = 1_000.0_f64;
    let t_end = 1e-2_f64;

    while start.elapsed().as_millis() < timelimit as u128 {
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        let ratio = elapsed / timelimit;
        let temp = t0 * (t_end / t0).powf(ratio);

        let op: f64 = rng.gen();
        if op < 0.15 && !order.is_empty() {
            // 削除
            let idx = rng.gen_range(0..order.len());
            let pt_rm = path[idx + 1];
            // Δ距離
            let delta = delta_len(&path, idx, None);
            let new_len = len + delta;
            if new_len <= M {
                let new_score = score - vortices[order[idx]].w;
                let accept = new_score > score
                    || rng.gen::<f64>() < ((new_score - score) as f64 / temp).exp();
                if accept {
                    // commit
                    path.remove(idx + 1);
                    order.remove(idx);
                    len = new_len;
                    score = new_score;
                    if score > best_score {
                        best_score = score;
                        best_path = path.clone();
                        best_order = order.clone();
                        best_cand_idx = cand_idx.clone();
                        println!("Score improved to {} at {:.0}ms", score, elapsed);
                    }
                }
            }
        } else if op < 0.35 && order.len() < vortices.len() {
            // 挿入（候補点をランダムに選択）
            let cand = rng.gen_range(0..vortices.len());
            if order.contains(&cand) {
                continue;
            }
            let cand_pt_idx = rng.gen_range(0..NUM_ANGLES);
            let pt_ins = vortices[cand].candidates[cand_pt_idx];
            if path.len() <= 1 { continue; }
            let idx = rng.gen_range(0..path.len() - 1);
            let delta = delta_len(&path, idx, Some(pt_ins));
            let new_len = len + delta;
            if new_len <= M {
                let new_score = score + vortices[cand].w;
                let accept = new_score > score
                    || rng.gen::<f64>() < ((new_score - score) as f64 / temp).exp();
                if accept {
                    path.insert(idx + 1, pt_ins);
                    order.insert(idx, cand);
                    cand_idx[cand] = cand_pt_idx;
                    len = new_len;
                    score = new_score;
                    if score > best_score {
                        best_score = score;
                        best_path = path.clone();
                        best_order = order.clone();
                        best_cand_idx = cand_idx.clone();
                        println!("Score improved to {} at {:.0}ms", score, elapsed);
                    }
                }
            }
        } else if op < 0.5 && !order.is_empty() {
            // 通過点の変更
            let idx = rng.gen_range(0..order.len());
            let vortex_idx = order[idx];
            let old_cand_idx = cand_idx[vortex_idx];
            let new_cand_idx = rng.gen_range(0..NUM_ANGLES);
            if old_cand_idx == new_cand_idx {
                continue;
            }
            
            let old_pt = vortices[vortex_idx].candidates[old_cand_idx];
            let new_pt = vortices[vortex_idx].candidates[new_cand_idx];
            
            // 距離の変化を計算
            let mut new_path = path.clone();
            new_path[idx + 1] = new_pt;
            let new_len = route_length(&new_path);
            
            if new_len <= M {
                let accept = new_len < len
                    || rng.gen::<f64>() < ((len - new_len) / temp).exp();
                if accept {
                    path = new_path;
                    cand_idx[vortex_idx] = new_cand_idx;
                    len = new_len;
                    if score > best_score {
                        best_score = score;
                        best_path = path.clone();
                        best_order = order.clone();
                        best_cand_idx = cand_idx.clone();
                        println!("Score improved to {} at {:.0}ms", score, elapsed);
                    }
                }
            }
        } else if op < 0.65 && order.len() >= 3 {
            // 3-opt
            let n = order.len();
            if n < 3 { continue; }
            let i = rng.gen_range(1..=n - 2);
            let j = rng.gen_range(i + 1..=n - 1);
            let k = rng.gen_range(j + 1..=n);
            
            // 最良のパターンを見つける
            let mut best_pattern = None;
            let mut best_delta = 0.0;
            for pattern in 0..4 {
                let delta = three_opt_delta(&path, i, j, k, pattern);
                if delta < best_delta {
                    best_delta = delta;
                    best_pattern = Some(pattern);
                }
            }
            
            if let Some(pattern) = best_pattern {
                let new_len = len + best_delta;
                if new_len <= M {
                    let accept = best_delta < 0.0
                        || rng.gen::<f64>() < (-best_delta / temp).exp();
                    if accept {
                        let mut new_path = path.clone();
                        let mut new_order = order.clone();
                        apply_three_opt(&mut new_path, &mut new_order, i, j, k, pattern);
                        path = new_path;
                        order = new_order;
                        len = new_len;
                    }
                }
            }
        } else if op < 0.8 && order.len() >= 2 {
            // Or-opt
            let segment_len = rng.gen_range(1..=3.min(order.len()));
            if path.len() <= segment_len + 2 { continue; }
            let from = rng.gen_range(1..=(path.len() - segment_len - 1));
            
            // 移動先の候補をいくつか試す
            let mut best_to = None;
            let mut best_delta = 0.0;
            
            for _ in 0..10 {
                let to = rng.gen_range(1..path.len());
                if to >= from && to < from + segment_len {
                    continue;
                }
                
                if let Some(delta) = or_opt_delta(&path, from, to, segment_len) {
                    if delta < best_delta {
                        best_delta = delta;
                        best_to = Some(to);
                    }
                }
            }
            
            if let Some(to) = best_to {
                let new_len = len + best_delta;
                if new_len <= M {
                    let accept = best_delta < 0.0
                        || rng.gen::<f64>() < (-best_delta / temp).exp();
                    if accept {
                        let mut new_path = path.clone();
                        let mut new_order = order.clone();
                        apply_or_opt(&mut new_path, &mut new_order, from, to, segment_len);
                        path = new_path;
                        order = new_order;
                        len = new_len;
                    }
                }
            }
        } else if order.len() >= 2 {
            // 2-opt swap
            let i = rng.gen_range(0..order.len() - 1);
            let j = rng.gen_range(i + 1..order.len());
            let mut new_path = path.clone();
            new_path[i + 1..=j + 1].reverse();
            let new_len = route_length(&new_path);
            if new_len <= M {
                let accept = new_len < len
                    || rng.gen::<f64>() < ((len - new_len) / temp).exp();
                if accept {
                    path = new_path;
                    order[i..=j].reverse();
                    len = new_len;
                    // score 不変
                }
            }
        }
    }

    // ④ 出力
    println!("{}", best_path.len() - 1);
    for (x, y) in best_path {
        println!("{} {}", x, y);
    }
}
