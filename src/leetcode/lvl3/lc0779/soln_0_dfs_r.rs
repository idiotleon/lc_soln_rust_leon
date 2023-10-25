/// @author: Leon
/// https://leetcode.com/problems/k-th-symbol-in-grammar/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        return Self::dfs(n, k, 0);
    }
    fn dfs(n: i32, k: i32, cur_root_val: i32) -> i32 {
        if n == 1 {
            return cur_root_val;
        }
        let total = 2_i32.pow(n as u32 - 1);
        let half = total / 2;
        if k > half {
            let nxt_root_val = if cur_root_val == 0 { 1 } else { 0 };
            return Self::dfs(n - 1, k - half, nxt_root_val);
        } else {
            let nxt_root_val = if cur_root_val == 0 { 0 } else { 1 };
            return Self::dfs(n - 1, k, nxt_root_val);
        }
    }
}
