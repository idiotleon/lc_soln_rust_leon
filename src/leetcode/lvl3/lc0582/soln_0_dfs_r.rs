use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/kill-process/
/// Time Complexity:    O(V + E)
/// Space Complexity:   O(V + E)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
        let len_ps: usize = pid.len();
        let graph: HashMap<usize, Vec<usize>> = Self::build_graph(pid, ppid);
        let mut ans: Vec<i32> = Vec::with_capacity(len_ps);
        Self::dfs(kill as usize, &graph, &mut ans);
        ans
    }
    fn dfs(cur: usize, graph: &HashMap<usize, Vec<usize>>, res: &mut Vec<i32>) {
        res.push(cur as i32);
        if let Some(nxts) = graph.get(&cur) {
            for &nxt in nxts {
                Self::dfs(nxt, graph, res);
            }
        }
    }
    fn build_graph(pid: Vec<i32>, ppid: Vec<i32>) -> HashMap<usize, Vec<usize>> {
        let len_ps: usize = pid.len();
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        for idx in 0..len_ps {
            let key: usize = ppid[idx] as usize;
            let val: usize = pid[idx] as usize;
            graph.entry(key).or_default().push(val);
        }
        graph
    }
}
