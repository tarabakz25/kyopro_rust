use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

// 2次元座標を扱うための構造体
#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x: x as f64, y: y as f64 }
    }
    fn dist_sq(&self, other: &Point) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
    fn sub(&self, other: &Point) -> Self {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
    fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

// 渦潮の情報を格納する構造体
struct Whirlpool {
    center: Point,
    w: i64,
}

/// 点と線分との最短距離の2乗を計算する
fn dist_sq_point_to_segment(p: &Point, a: &Point, b: &Point) -> f64 {
    if a.dist_sq(b) < 1e-9 { return p.dist_sq(a); }
    let ab = b.sub(a);
    let ap = p.sub(a);
    let t = ap.dot(&ab) / ab.dot(&ab);
    if t < 0.0 { p.dist_sq(a) } 
    else if t > 1.0 { p.dist_sq(b) } 
    else {
        let projection = Point { x: a.x + t * ab.x, y: a.y + t * ab.y };
        p.dist_sq(&projection)
    }
}

/// 経路と渦潮の情報から合計スコアを計算する
fn calculate_score_sum(path: &Vec<Point>, whirlpools: &Vec<Whirlpool>, r: f64) -> i64 {
    let mut total_w = 0;
    let r_sq = r * r;
    let r2_sq = (2.0 * r).powi(2);

    for whirlpool in whirlpools {
        let mut min_dist_sq_to_path = f64::MAX;
        if path.len() >= 2 {
            for segment in path.windows(2) {
                let dist_sq = dist_sq_point_to_segment(&whirlpool.center, &segment[0], &segment[1]);
                if dist_sq < min_dist_sq_to_path {
                    min_dist_sq_to_path = dist_sq;
                }
            }
        }
        if min_dist_sq_to_path < r_sq { continue; }
        if min_dist_sq_to_path <= r2_sq { total_w += whirlpool.w; }
    }
    total_w
}

/// 問題入力ファイルを読み込む
fn read_problem_input(path: &str) -> io::Result<(Vec<Whirlpool>, f64)> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file).lines();

    // 1行目: N L M R
    let first_line = reader.next().unwrap()?.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let n = first_line[0] as usize;
    let r = first_line[3] as f64;

    // 2行目以降: 渦潮データ
    let mut whirlpools = Vec::with_capacity(n);
    for line in reader.take(n) {
        let parts = line?.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        whirlpools.push(Whirlpool {
            center: Point::new(parts[0], parts[1]),
            w: parts[2],
        });
    }
    Ok((whirlpools, r))
}

/// 解答出力ファイルを読み込む
fn read_solution_output(path: &str) -> io::Result<Vec<Point>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file).lines();
    
    // 1行目: k
    let k = reader.next().unwrap()?.parse::<usize>().unwrap();

    // 2行目以降: 経由地
    let mut path = Vec::with_capacity(k + 1);
    for line in reader.take(k + 1) {
        let parts = line?.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        path.push(Point::new(parts[0], parts[1]));
    }
    Ok(path)
}


fn main() {
    // 1. コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("実行方法: cargo run -- <入力ファイルパス> <出力ファイルパス>");
        process::exit(1);
    }
    let input_path = &args[1];
    let output_path = &args[2];

    // 2. ファイルを読み込む
    let (whirlpools, r) = read_problem_input(input_path).expect("入力ファイルの読み込みに失敗しました");
    let path = read_solution_output(output_path).expect("出力ファイルの読み込みに失敗しました");

    // 3. スコアを計算して表示
    let score_sum = calculate_score_sum(&path, &whirlpools, r);
    
    // 問題文に従い、有効な経路であれば最終スコアは +1 される
    let final_score = score_sum + 1;

    println!("読み込んだ経路の経由地数: {}", path.len());
    println!("獲得した感動ポイントの合計: {}", score_sum);
    println!("最終スコア: {}", final_score);
}
