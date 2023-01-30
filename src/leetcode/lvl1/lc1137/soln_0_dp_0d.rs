/// @author: Leon
/// https://leetcode.com/problems/n-th-tribonacci-number/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 1;
        }
        let mut t0: i32 = 0;
        let mut t1: i32 = 1;
        let mut t2: i32 = 1;
        let mut ans: i32 = 0;
        let mut tn: i32 = 2;
        while tn < n {
            ans = t0 + t1 + t2;
            t0 = t1;
            t1 = t2;
            t2 = ans;
            tn += 1;
        }
        return ans;
    }
}
