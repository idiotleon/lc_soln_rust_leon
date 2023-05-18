/// @author: Leon
/// https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes/
/// Time Complexity:    O(`_len_es`)
/// Space Complexity:   O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let _len_es: usize = edges.len();
        let degrees = {
            let mut degrees = vec![0; n as usize];
            for edge in edges {
                let to = edge[1];
                degrees[to as usize] += 1;
            }
            degrees
        };
        let mut ans: Vec<i32> = Vec::with_capacity(n as usize);
        for idx in 0..n as usize {
            if degrees[idx] == 0 {
                ans.push(idx as i32);
            }
        }
        return ans;
    }
}
