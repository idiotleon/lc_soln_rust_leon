/// @author: Leon
/// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_cns`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_cns`)
/// Reference:
/// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/discuss/827414/straightforward-c++-solution-oror-bfs/1104021
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
        let mut visited: Vec<bool> = vec![false; n];
        let mut cnt: i32 = 0;
        Self::dfs(0, &mut visited, &adj, &back, &mut cnt);
        cnt
    }
    fn dfs(
        cur: usize,
        visited: &mut Vec<bool>,
        adj: &Vec<Vec<usize>>,
        back: &Vec<Vec<usize>>,
        cnt: &mut i32,
    ) {
        visited[cur] = true;
        for &a in &adj[cur] {
            if visited[a] {
                continue;
            }
            *cnt += 1;
            Self::dfs(a, visited, adj, back, cnt);
        }
        for &b in &back[cur] {
            if visited[b] {
                continue;
            }
            Self::dfs(b, visited, adj, back, cnt);
        }
    }
}
