/// @author: Leon
/// https://leetcode.com/problems/concatenation-of-array/
///
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1) / O(`len_n`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let len_n = nums.len();
        let ans: Vec<i32> = {
            let mut res = vec![0; 2 * len_n];
            for (idx, &num) in nums.iter().enumerate() {
                res[idx] = num;
                res[idx + len_n] = num;
            }
            res
        };
        ans
    }
}
