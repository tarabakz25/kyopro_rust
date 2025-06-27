use std::{
  io::{stdout, Write, Result},
  thread::sleep,
  time::{Duration, Instant},
};

use crossterm::{
  cursor,
  terminal::{self, ClearType},
  ExecutableCommand, QueueableCommand,
};

/// 3D 座標
#[derive(Copy, Clone)]
struct Vec3(f32, f32, f32);

impl Vec3 {
  /// 回転（ラジアン）
  fn rotate(&self, ax: f32, ay: f32, az: f32) -> Self {
      // X 回転
      let (sin_x, cos_x) = ax.sin_cos();
      let mut v = Vec3(
          self.0,
          self.1 * cos_x - self.2 * sin_x,
          self.1 * sin_x + self.2 * cos_x,
      );

      // Y 回転
      let (sin_y, cos_y) = ay.sin_cos();
      v = Vec3(
          v.0 * cos_y + v.2 * sin_y,
          v.1,
          -v.0 * sin_y + v.2 * cos_y,
      );

      // Z 回転
      let (sin_z, cos_z) = az.sin_cos();
      Vec3(
          v.0 * cos_z - v.1 * sin_z,
          v.0 * sin_z + v.1 * cos_z,
          v.2,
      )
  }
}

/// 立方体の 8 頂点（中心=(0,0,0)、一辺=2）
const VERTICES: [Vec3; 8] = [
  Vec3(-1.0, -1.0, -1.0),
  Vec3(-1.0, -1.0,  1.0),
  Vec3(-1.0,  1.0, -1.0),
  Vec3(-1.0,  1.0,  1.0),
  Vec3( 1.0, -1.0, -1.0),
  Vec3( 1.0, -1.0,  1.0),
  Vec3( 1.0,  1.0, -1.0),
  Vec3( 1.0,  1.0,  1.0),
];

/// 頂点インデックスで表す 12 辺
const EDGES: [(usize, usize); 12] = [
  (0, 1), (0, 2), (0, 4),
  (1, 3), (1, 5),
  (2, 3), (2, 6),
  (3, 7),
  (4, 5), (4, 6),
  (5, 7),
  (6, 7),
];

fn main() -> Result<()> {
  // 端末サイズ取得
  let (cols, rows) = terminal::size()?;
  let mut stdout = stdout();
  terminal::enable_raw_mode()?;            // カーソル点滅を抑止
  stdout.execute(cursor::Hide)?;           // カーソル非表示

  // 60 FPS 相当
  let frame_time = Duration::from_millis(16);
  let mut t0 = Instant::now();

  let mut angle: f32 = 0.0;

  loop {
      // ★ Ctrl-C で即終了したい場合はここにハンドリングを入れる
      if crossterm::event::poll(Duration::from_millis(0))? {
          if let crossterm::event::Event::Key(k) = crossterm::event::read()? {
              if k.code == crossterm::event::KeyCode::Char('q') {
                  break;
              }
          }
      }

      // 画面バッファ (rows × cols) をスペースで初期化
      let mut buffer = vec![b' '; (rows * cols) as usize];

      // 回転角
      angle += 0.02;
      let ax = angle;
      let ay = angle * 0.7;
      let az = angle * 1.3;

      // 各辺を直線近似で描画
      for (i0, i1) in EDGES {
          let p0 = project(VERTICES[i0].rotate(ax, ay, az), cols, rows);
          let p1 = project(VERTICES[i1].rotate(ax, ay, az), cols, rows);
          draw_line(&mut buffer, cols, rows, p0, p1, b'#');
      }

      // 描画
      stdout
          .queue(cursor::MoveTo(0, 0))?
          .queue(terminal::Clear(ClearType::All))?;
      // UTF-8 化して一括出力
      let frame = buffer
          .chunks(cols as usize)
          .map(|line| std::str::from_utf8(line).unwrap())
          .collect::<Vec<_>>()
          .join("\n");
      stdout.queue(crossterm::style::Print(frame))?;
      stdout.flush()?;

      // FPS 制御
      let elapsed = t0.elapsed();
      if elapsed < frame_time {
          sleep(frame_time - elapsed);
      }
      t0 = Instant::now();
  }

  // 後片付け
  stdout.execute(cursor::Show)?;
  terminal::disable_raw_mode()?;
  Ok(())
}

/// 3D → 2D 透視投影
fn project(v: Vec3, cols: u16, rows: u16) -> (i32, i32) {
  let scale = 8.0;         // サイズ調整
  let fov = 3.0;           // 視野角 (視点距離)
  let z = v.2 + fov;       // z が大きいほど遠い
  let x = (v.0 / z) * scale + (cols as f32) / 2.0;
  let y = (v.1 / z) * scale + (rows as f32) / 2.0;
  (x as i32, y as i32)
}

/// 終端座標 (x,y) に '#' を線形補間で描く
fn draw_line(buf: &mut [u8], cols: u16, rows: u16, p0: (i32, i32), p1: (i32, i32), ch: u8) {
  let (mut x0, mut y0) = p0;
  let (x1, y1) = p1;
  let dx = (x1 - x0).abs();
  let sx = if x0 < x1 { 1 } else { -1 };
  let dy = -(y1 - y0).abs();
  let sy = if y0 < y1 { 1 } else { -1 };
  let mut err = dx + dy;
  loop {
      put_char(buf, cols, rows, x0, y0, ch);
      if x0 == x1 && y0 == y1 {
          break;
      }
      let e2 = 2 * err;
      if e2 >= dy {
          err += dy;
          x0 += sx;
      }
      if e2 <= dx {
          err += dx;
          y0 += sy;
      }
  }
}

/// バッファに 1 文字書き込む（範囲外は無視）
fn put_char(buf: &mut [u8], cols: u16, rows: u16, x: i32, y: i32, ch: u8) {
  if x >= 0 && x < cols as i32 && y >= 0 && y < rows as i32 {
      let idx = (y as usize) * (cols as usize) + (x as usize);
      buf[idx] = ch;
  }
}
