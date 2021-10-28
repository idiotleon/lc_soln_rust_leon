/// @author: Leon
/// https://leetcode.com/problems/fibonacci-number/
///
/// Time Complexity:     O(`n`)
/// Space Complexity:    O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut fn_2 = 0;
        let mut fn_1 = 1;
        let mut sum = 1;
        for _ in 2..n + 1 {
            sum = fn_2 + fn_1;
            fn_2 = fn_1;
            fn_1 = sum;
        }
        sum
    }
}
