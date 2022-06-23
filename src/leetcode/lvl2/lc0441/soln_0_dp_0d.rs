/// @author: Leon
/// https://leetcode.com/problems/arranging-coins/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut steps = 1;
        let mut sum: u64 = 0;
        let n = n as u64;
        while sum < n {
            steps += 1;
            sum += steps;
        }
        steps as i32 - 1
    }
}
