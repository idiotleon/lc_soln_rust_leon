/// @author: Leon
/// https://leetcode.com/problems/time-needed-to-inform-all-employees/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_mgrs`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_mgrs`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, managers: Vec<i32>, inform_times: Vec<i32>) -> i32 {
        let _len_mgrs: usize = managers.len();
        let graph: Vec<Vec<usize>> = Self::build_graph(n as usize, &managers);
        Self::dfs(head_id as usize, &inform_times, &graph)
    }
    fn dfs(mgr: usize, inform_times: &Vec<i32>, graph: &Vec<Vec<usize>>) -> i32 {
        let mut max: i32 = 0;
        for &employee in &graph[mgr] {
            max = std::cmp::max(max, Self::dfs(employee, inform_times, graph));
        }
        max + inform_times[mgr]
    }
    fn build_graph(len_vts: usize, managers: &Vec<i32>) -> Vec<Vec<usize>> {
        let mut graph: Vec<Vec<usize>> = vec![vec![]; len_vts];
        for idx in 0..len_vts {
            let mgr = managers[idx];
            if mgr != -1 {
                graph[mgr as usize].push(idx);
            }
        }
        graph
    }
}
