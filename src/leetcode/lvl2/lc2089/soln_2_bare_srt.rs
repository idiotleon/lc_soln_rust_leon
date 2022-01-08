/// @author: Leon
/// https://leetcode.com/problems/find-target-indices-after-sorting-array/
/// Time Complexity:    O(`_len_n` * lg(`_len_n`)) + O(`_len_n`) ~ O(`_len_n`)
/// Space Complexity:   O(1) / O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let _len_n: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut sorted: Vec<i32> = nums;
            sorted.sort();
            sorted
        };
        let ans: Vec<i32> = {
            let mut res: Vec<i32> = Vec::new();
            for (idx, num) in sorted.into_iter().enumerate() {
                if num == target {
                    res.push(idx as i32);
                }
            }
            res
        };
        ans
    }
}
