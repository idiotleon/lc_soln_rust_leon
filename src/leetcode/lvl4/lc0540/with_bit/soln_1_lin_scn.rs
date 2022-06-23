/// @author: Leon
/// https://leetcode.com/problems/single-element-in-a-sorted-array/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let _len_n = nums.len();
        let ans = {
            let mut ans = 0;
            for num in nums {
                ans ^= num;
            }
            ans
        };
        ans
    }
}
