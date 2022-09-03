/// @author: Leon
/// https://leetcode.com/problems/count-strictly-increasing-subarrays/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i64 {
        let len_ns: usize = nums.len();
        let mut cnt: i64 = 1;
        let mut len: i64 = 1;
        for idx in 1..len_ns {
            if nums[idx - 1] < nums[idx] {
                len += 1;
                cnt += len;
            } else {
                len = 1;
                cnt += 1;
            }
        }
        return cnt;
    }
}
