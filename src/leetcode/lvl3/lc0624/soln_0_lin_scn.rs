/// @author: Leon
/// https://leetcode.com/problems/maximum-distance-in-arrays/
/// Time Complexity:    O(`_len_as`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_distance(arrs: Vec<Vec<i32>>) -> i32 {
        const RANGE: i32 = 1e4 as i32 + 7;
        const RANGE_IDX: usize = 500 + 1;
        let _len_as: usize = arrs.len();
        let mut min: i32 = RANGE;
        let mut min_idx: usize = RANGE_IDX;
        let mut min_sec: i32 = RANGE;
        let mut max: i32 = -RANGE;
        let mut max_idx: usize = RANGE_IDX;
        let mut max_sec: i32 = -RANGE;
        for (idx, nums) in arrs.into_iter().enumerate() {
            let len_ns = nums.len();
            if nums[0] < min {
                min_sec = min;
                min = nums[0];
                min_idx = idx;
            } else if nums[0] < min_sec {
                min_sec = nums[0];
            }
            if nums[len_ns - 1] > max {
                max_sec = max;
                max = nums[len_ns - 1];
                max_idx = idx;
            } else if nums[len_ns - 1] > max_sec {
                max_sec = nums[len_ns - 1];
            }
        }
        return if min_idx == max_idx {
            std::cmp::max((max_sec - min).abs(), (max - min_sec).abs())
        } else {
            max - min
        };
    }
}
