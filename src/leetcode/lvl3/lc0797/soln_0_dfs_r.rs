/// @author: Leon
/// https://leetcode.com/problems/all-paths-from-source-to-target/
/// Time Complexity:    O(V + E)
/// Space Complexity:   O(V + E)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len_n = graph.len();
        let ans = {
            let mut paths: Vec<Vec<i32>> = Vec::new();
            let mut path: Vec<i32> = vec![0];
            Self::dfs(0, &mut path, &graph, len_n - 1, &mut paths);
            paths
        };
        ans
    }
    fn dfs(
        cur: usize,
        path: &mut Vec<i32>,
        graph: &Vec<Vec<i32>>,
        destination: usize,
        paths: &mut Vec<Vec<i32>>,
    ) {
        if cur == destination {
            paths.push(path.to_vec());
            return;
        }
        for &next in &graph[cur] {
            path.push(next);
            Self::dfs(next as usize, path, graph, destination, paths);
            path.pop();
        }
    }
}
