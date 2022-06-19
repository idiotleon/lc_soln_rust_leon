/// @author: Leon
/// https://leetcode.com/problems/sum-of-numbers-with-units-digit-k/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_numbers(sum: i32, k: i32) -> i32 {
        if sum == 0 {
            return 0;
        }
        for idx_digit in 1..=10 {
            if (idx_digit * k) % 10 == sum % 10 && idx_digit * k <= sum {
                return idx_digit;
            }
        }
        -1
    }
}
