/// @author: Leon
/// https://leetcode.com/problems/frequency-of-the-most-frequent-element/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/frequency-of-the-most-frequent-element/?envType=daily-question&envId=2023-11-18
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let sorted = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        let mut cur_sum: i64 = 0;
        while hi < len_ns {
            let target = sorted[hi];
            cur_sum += target as i64;
            if (hi - lo + 1) as i64 * target as i64 - cur_sum > k as i64 {
                cur_sum -= sorted[lo] as i64;
                lo += 1;
            }
            hi += 1;
        }
        return (len_ns - lo) as i32;
    }
}
