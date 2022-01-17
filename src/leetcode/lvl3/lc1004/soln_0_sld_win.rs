/// @author: Leon
/// https://leetcode.com/problems/max-consecutive-ones-iii/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let len_n: usize = nums.len();
        let mut k = k;
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        while hi < len_n {
            if nums[hi] == 0 {
                k -= 1;
            }
            if k < 0 {
                if nums[lo] == 0 {
                    k += 1;
                }
                lo += 1;
            }
            hi += 1;
        }
        (hi - lo) as i32
    }
}
