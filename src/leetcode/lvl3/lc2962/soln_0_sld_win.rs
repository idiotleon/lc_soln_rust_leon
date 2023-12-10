/// @author: Leon
/// https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let len_ns: usize = nums.len();
        let max: i32 = *nums.iter().max().unwrap();
        let mut cnt_max: i32 = 0;
        let mut cnt: i64 = 0;
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        while hi < len_ns {
            if nums[hi] == max {
                cnt_max += 1;
            }
            while cnt_max >= k {
                if nums[lo] == max {
                    cnt_max -= 1;
                }
                lo += 1;
            }
            cnt += lo as i64;
            hi += 1;
        }
        return cnt;
    }
}
