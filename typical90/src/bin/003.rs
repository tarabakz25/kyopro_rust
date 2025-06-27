use proconio::input;
use std::collections::VecDeque;

fn bfs(start: usize, graph: &[Vec<usize>]) -> (usize, Vec<usize>) {
    let n = graph.len();
    let mut dist = vec![usize::MAX; n];
    let mut q = VecDeque::new();

    dist[start] = 0;
    q.push_back(start);

    while let Some(v) = q.pop_front() {
        for &nv in &graph[v] {
            if dist[nv] == usize::MAX {
                dist[nv] = dist[v] + 1;
                q.push_back(nv);
            }
        }
    }

    let far = (0..n).max_by_key(|&i| dist[i]).unwrap();

    (dist[far], dist)
}

fn main() {
    input! {
        n: usize,
        e: [(usize, usize); n-1],
    }

    let mut graph = vec![vec![]; n+1];

    for (a, b) in e {
        let a = a - 1;
        let b = b - 1;

        graph[a].push(b);
        graph[b].push(a);
    }

    let (a, _) = bfs(0, &graph);
    let (_, dist) = bfs(a, &graph);

    let diameter = *dist.iter().max().unwrap();

    println!("{}", diameter);
}