/// @author: Leon
/// https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph/
/// Time Complexity:    O(V * (V + E)) ~ O(`n` * (`n` + `_len_es`))
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_es`)
/// Reference:
/// https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph/discuss/1822108/C%2B%2B-Solution-or-Plain-DFS-or-Short-and-Clean-code
/// Note:
/// This is a raw recursive DFS approach.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let _len_es: usize = edges.len();
        let n: usize = n as usize;
        let graph: Vec<Vec<usize>> = Self::build_graph(n, &edges);
        let mut ans: Vec<Vec<i32>> = vec![Vec::new(); n];
        for cur in 0..n {
            let mut visited: Vec<bool> = vec![false; n];
            Self::dfs(cur, &mut visited, cur, &graph, &mut ans);
        }
        return ans;
    }
    fn dfs(
        cur: usize,
        visited: &mut Vec<bool>,
        prev: usize,
        graph: &Vec<Vec<usize>>,
        res: &mut Vec<Vec<i32>>,
    ) {
        visited[cur] = true;
        for &nxt in &graph[cur] {
            if visited[nxt] {
                continue;
            }
            res[nxt].push(prev as i32);
            Self::dfs(nxt, visited, prev, graph, res);
        }
    }
    fn build_graph(n: usize, edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
        for edge in edges {
            let from: usize = edge[0] as usize;
            let to: usize = edge[1] as usize;
            graph[from].push(to);
        }
        return graph;
    }
}
