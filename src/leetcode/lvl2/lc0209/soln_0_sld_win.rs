/// @author: Leon
/// https://leetcode.com/problems/minimum-size-subarray-sum/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        const RANGE: usize = 1e5 as usize + 1;
        let mut sum: i32 = 0;
        let mut hi: usize = 0;
        let mut lo: usize = 0;
        let mut shortest: usize = RANGE;
        while hi < len_n {
            sum += nums[hi];
            // the condition, `lo <= hi`, can be omitted
            while lo <= hi && sum >= target {
                let len = hi - lo + 1;
                shortest = std::cmp::min(shortest, len);
                sum -= nums[lo];
                lo += 1;
            }
            hi += 1;
        }
        if shortest == RANGE {
            0
        } else {
            shortest as i32
        }
    }
}
