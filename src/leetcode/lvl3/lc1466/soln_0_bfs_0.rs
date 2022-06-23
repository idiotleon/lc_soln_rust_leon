use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_cns`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_cns`)
/// Reference:
/// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/discuss/827414/straightforward-c%2B%2B-solution-oror-bfs
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n: usize = n as usize;
        let _len_cns: usize = connections.len();
        let (adj, back): (Vec<Vec<usize>>, Vec<Vec<usize>>) = {
            let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
            let mut back: Vec<Vec<usize>> = vec![vec![]; n];
            for conn in connections {
                let u: usize = conn[0] as usize;
                let v: usize = conn[1] as usize;
                adj[u].push(v);
                back[v].push(u);
            }
            (adj, back)
        };
        let mut queue: VecDeque<usize> = {
            let mut queue: VecDeque<usize> = VecDeque::new();
            queue.push_back(0);
            queue
        };
        let mut visited: Vec<bool> = vec![false; n];
        let mut cnt: i32 = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    visited[cur] = true;
                    for &a in &adj[cur] {
                        if visited[a] {
                            continue;
                        }
                        cnt += 1;
                        queue.push_back(a);
                    }
                    for &b in &back[cur] {
                        if visited[b] {
                            continue;
                        }
                        queue.push_back(b);
                    }
                }
            }
        }
        cnt
    }
}
