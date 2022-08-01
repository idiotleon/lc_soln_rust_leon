/// @author: Leon
/// https://leetcode.com/problems/unique-paths/
/// Time Complexity:     O(`m` * `n`)
/// Space Complexity:    O(`n`)
/// Reference:
/// https://leetcode.com/problems/unique-paths/discuss/22954/C%2B%2B-DP
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut prev = vec![1; n];
        let mut cur = vec![1; n];
        for _r in 1..m {
            for c in 1..n {
                cur[c] = prev[c] + cur[c - 1];
            }
            prev = cur.clone();
        }
        return prev[n - 1];
    }
}
