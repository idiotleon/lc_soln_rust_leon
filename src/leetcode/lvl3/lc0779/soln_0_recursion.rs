/// @author: Leon
/// https://leetcode.com/problems/k-th-symbol-in-grammar/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/k-th-symbol-in-grammar/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        return Self::recurse(n, k);
    }
    fn recurse(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        let total = 2_i32.pow(n as u32 - 1);
        let half = total / 2;
        if k > half {
            return 1 - Self::recurse(n, k - half);
        }
        return Self::recurse(n - 1, k);
    }
}
