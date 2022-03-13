/// @author: Leon
/// https://leetcode.com/problems/find-three-consecutive-integers-that-sum-to-a-given-number/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        if num % 3 == 0 {
            let mid = num / 3;
            return vec![mid - 1, mid, mid + 1];
        }
        vec![]
    }
}
