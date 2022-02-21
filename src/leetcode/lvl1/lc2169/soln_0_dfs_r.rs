/// @author: Leon
/// https://leetcode.com/problems/count-operations-to-obtain-zero/
/// Time Complexity:    O(lg(min(`num1`, `num2`)))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        Self::dfs(num1, num2)
    }
    fn dfs(lo: i32, hi: i32) -> i32 {
        if lo > hi {
            return Self::dfs(hi, lo);
        }
        if lo == 0 {
            return 0;
        }
        if lo == hi {
            return 1;
        }
        let mut cnt: i32 = 0;
        let res = hi - lo;
        cnt += 1;
        cnt += Self::dfs(res, lo);
        cnt
    }
}
