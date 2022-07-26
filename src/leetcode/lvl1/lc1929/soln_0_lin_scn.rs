/// @author: Leon
/// https://leetcode.com/problems/concatenation-of-array/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1) / O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let len_ns = nums.len();
        let ans: Vec<i32> = {
            let mut res = vec![0; 2 * len_ns];
            for (idx, &num) in nums.iter().enumerate() {
                res[idx] = num;
                res[idx + len_ns] = num;
            }
            res
        };
        ans
    }
}
