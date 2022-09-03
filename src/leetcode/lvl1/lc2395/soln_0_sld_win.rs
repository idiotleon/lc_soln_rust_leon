/// @author: Leon
/// https://leetcode.com/problems/find-subarrays-with-equal-sum/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
use std::collections::HashSet;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let len_ns: usize = nums.len();
        let mut sum: i32 = 0;
        let mut seen: HashSet<i32> = HashSet::with_capacity(len_ns);
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        while hi < len_ns {
            sum += nums[hi];
            if hi - lo + 1 == 2 {
                if !seen.insert(sum) {
                    return true;
                }
                sum -= nums[lo];
                lo += 1;
            }
            hi += 1;
        }
        return false;
    }
}
