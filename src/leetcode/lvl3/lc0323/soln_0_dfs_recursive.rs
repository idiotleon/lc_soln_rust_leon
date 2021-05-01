// @author: Leon
// https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/
//
// Time Complexity:     O(V + E)
// Space Complexity:    O(V + E)
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let graph = Self::build_graph(n, &edges);
        let mut visited = vec![0; n];
        let mut cnt = 0;

        for i in 0..n {
            if visited[i] == 1 {
                continue;
            }

            Self::dfs(i, &mut visited, &graph);
            cnt += 1;
        }

        cnt
    }

    fn dfs(cur: usize, mut visited: &mut Vec<i32>, graph: &Vec<Vec<usize>>) {
        if visited[cur] == 1 {
            return;
        }

        visited[cur] = 1;

        for &next in graph[cur].iter() {
            Self::dfs(next, &mut visited, &graph);
        }
    }

    fn build_graph(n: usize, edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(n); n];

        for edge in edges.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;

            graph[u].push(v);
            graph[v].push(u);
        }

        graph
    }
}