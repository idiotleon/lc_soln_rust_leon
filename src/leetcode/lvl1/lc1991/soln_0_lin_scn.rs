/// @author: Leon
/// https://leetcode.com/problems/find-the-middle-index-in-array/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let _len_n: usize = nums.len();
        let sum: i32 = nums.iter().sum();
        let mut sum_cur: i32 = 0;
        for (idx, num) in nums.into_iter().enumerate() {
            if sum_cur == sum - sum_cur - num {
                return idx as i32;
            }
            sum_cur += num;
        }
        -1
    }
}
