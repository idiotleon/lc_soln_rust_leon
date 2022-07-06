/// @author: Leon
/// https://leetcode.com/problems/fibonacci-number/
/// Time Complexity:     O(`n`)
/// Space Complexity:    O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut prev: i32 = 0;
        let mut cur: i32 = 1;
        for _ in 1..n {
            let sum = prev + cur;
            prev = cur;
            cur = sum;
        }
        cur
    }
}
