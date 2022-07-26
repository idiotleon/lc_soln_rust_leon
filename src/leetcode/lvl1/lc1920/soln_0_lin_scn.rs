/// @author: Leon
/// https://leetcode.com/problems/build-array-from-permutation/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1) / O(`len_ns`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let len_ns = nums.len();
        let ans: Vec<i32> = {
            let mut tmp = vec![0; len_ns];
            for (idx, &num) in nums.iter().enumerate() {
                tmp[idx] = nums[num as usize];
            }
            tmp
        };
        ans
    }
}
