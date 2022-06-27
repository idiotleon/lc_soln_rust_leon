/// @author: Leon
/// https://leetcode.com/problems/find-eventual-safe-states/
/// Time Complexity:    O(V + E)
/// Space Complexity:   O(V + E)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const SAFE: u8 = 1;
    const UNSAFE: u8 = 2;
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let len_vts: usize = graph.len();
        let mut states: Vec<Option<u8>> = vec![None; len_vts];
        let mut ans: Vec<i32> = Vec::with_capacity(len_vts);
        for idx in 0..len_vts {
            if Self::is_safe(idx, &mut states, &graph) {
                ans.push(idx as i32);
            }
        }
        ans
    }
    fn is_safe(cur: usize, states: &mut Vec<Option<u8>>, graph: &Vec<Vec<i32>>) -> bool {
        if let Some(state) = states[cur] {
            return state == Self::SAFE;
        }
        states[cur] = Some(Self::UNSAFE);
        for &nxt in &graph[cur] {
            if !Self::is_safe(nxt as usize, states, graph) {
                return false;
            }
        }
        states[cur] = Some(Self::SAFE);
        true
    }
}
