/// @author: Leon
/// https://leetcode.com/problems/sum-of-all-odd-length-subarrays/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/sum-of-all-odd-length-subarrays/solutions/854184/java-c-python-o-n-time-o-1-space/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sum_odd_length_subarrays(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let mut cnt: i32 = 0;
        for idx in 0..len_ns {
            cnt += ((idx + 1) as i32 * (len_ns - idx) as i32 + 1) / 2 * nums[idx];
        }
        return cnt;
    }
}
