/// @author: Leon
/// https://leetcode.com/problems/maximize-the-topmost-element-after-k-moves/
/// Time Complexity:    O(`k`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/maximize-the-topmost-element-after-k-moves/discuss/1844102/C%2B%2B-Discuss-case-by-case
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
        let len_n: usize = nums.len();
        let k: usize = k as usize;
        if k == 0 {
            return if len_n >= 1 { nums[0] } else { -1 };
        };
        if k == 1 {
            return if len_n == 1 { -1 } else { nums[1] };
        };
        if len_n == 1 {
            return if k % 2 == 0 { nums[0] } else { -1 };
        };
        let max: i32 = {
            let mut max: i32 = 0;
            for idx in 0..std::cmp::min(k - 1, len_n) {
                max = std::cmp::max(max, nums[idx]);
            }
            if k < len_n {
                max = std::cmp::max(max, nums[k]);
            }
            max
        };
        max
    }
}
