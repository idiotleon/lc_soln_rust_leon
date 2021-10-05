/// @author: Leon
/// https://leetcode.com/problems/climbing-stairs/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut prev_one: i32 = 1;
        let mut prev_two: i32 = 0;
        let mut cur: i32 = 0;
        for _stp in 1..=n {
            cur = prev_one + prev_two;
            prev_two = prev_one;
            prev_one = cur;
        }
        cur
    }
}
