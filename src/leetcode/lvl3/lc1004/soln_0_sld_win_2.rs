/// @author: Leon
/// https://leetcode.com/problems/max-consecutive-ones-iii/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/max-consecutive-ones-iii/discuss/247564/JavaC%2B%2BPython-Sliding-Window
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let mut k = k;
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        while hi < len_ns {
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
        return (hi - lo) as i32;
    }
}
