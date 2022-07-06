/// @author: Leon
/// https://leetcode.com/problems/fibonacci-number/
/// Time Complexity:     O(`n`)
/// Space Complexity:    O(`n`) ~ O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let len_ns: usize = n as usize;
        let mut dp: Vec<i32> = vec![0; 1 + len_ns];
        dp[0] = 0;
        dp[1] = 1;
        for idx in 2..=len_ns {
            dp[idx] = dp[idx - 1] + dp[idx - 2];
        }
        dp[len_ns]
    }
}
