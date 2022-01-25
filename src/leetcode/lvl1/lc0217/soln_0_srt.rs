/// @author: Leon
/// https://leetcode.com/problems/contains-duplicate/
/// Time Complexity:    O(`len_n` * lg(`len_n`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let len_n: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut sorted = nums;
            sorted.sort();
            sorted
        };
        for idx in 1..len_n {
            if sorted[idx - 1] == sorted[idx] {
                return true;
            }
        }
        false
    }
}
