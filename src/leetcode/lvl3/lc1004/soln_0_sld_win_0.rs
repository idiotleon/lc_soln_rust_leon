/// @author: Leon
/// https://leetcode.com/problems/max-consecutive-ones-iii/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/max-consecutive-ones-iii/discuss/1304883/C%2B%2BPython-3-solutions-Easy-to-understand-Clean-and-Concise
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let len_n: usize = nums.len();
        let mut longest: usize = 0;
        let mut cnt: i32 = 0;
        let mut hi: usize = 0;
        let mut lo: usize = 0;
        while hi < len_n {
            if nums[hi] == 0 {
                cnt += 1;
                while cnt > k {
                    if nums[lo] == 0 {
                        cnt -= 1;
                    }
                    lo += 1;
                }
            }
            let len = hi - lo + 1;
            longest = std::cmp::max(longest, len);
            hi += 1;
        }
        longest as i32
    }
}
