/// @author: Leon
/// https://leetcode.com/problems/number-of-zero-filled-subarrays/
/// Time Complexity:    O(`_len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let _len_ns: usize = nums.len();
        let mut cnt: i64 = 0;
        let mut len: i64 = 0;
        for num in nums {
            if num == 0 {
                len += 1;
                cnt += len;
            } else {
                len = 0;
            }
        }
        cnt
    }
}
