/// @author: Leon
/// https://leetcode.com/problems/course-schedule/
/// Time Complexity:    O(V + E) ~ O(`num_course` + `_len_edges`)
/// Space Complexity:   O(V + E) ~ O(`num_course` + `_len_edges`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let len_vts: usize = num_courses as usize;
        let _len_edges: usize = prerequisites.len();
        let graph: Vec<Vec<usize>> = Self::build_graph(len_vts, &prerequisites);
        let mut visited: Vec<Option<bool>> = vec![None; len_vts];
        let mut memo: Vec<Option<bool>> = vec![None; len_vts];
        for course in 0..num_courses {
            if !Self::dfs(course as usize, &mut visited, &graph, &mut memo) {
                return false;
            }
        }
        true
    }
    fn dfs(
        cur: usize,
        visited: &mut Vec<Option<bool>>,
        graph: &Vec<Vec<usize>>,
        memo: &mut Vec<Option<bool>>,
    ) -> bool {
        if let Some(v) = visited[cur] {
            if v {
                return false;
            }
        }
        if let Some(m) = memo[cur] {
            if m {
                return true;
            }
        }
        visited[cur] = Some(true);
        for &nxt in &graph[cur] {
            if !Self::dfs(nxt, visited, graph, memo) {
                return false;
            }
        }
        visited[cur] = Some(false);
        memo[cur] = Some(true);
        true
    }
    fn build_graph(len_vts: usize, edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut graph: Vec<Vec<usize>> = vec![vec![]; len_vts];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[v].push(u);
        }
        graph
    }
}
