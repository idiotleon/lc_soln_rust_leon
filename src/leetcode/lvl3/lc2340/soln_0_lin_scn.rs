/// @author: Leon
/// https://leetcode.com/problems/minimum-adjacent-swaps-to-make-a-valid-array/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/minimum-adjacent-swaps-to-make-a-valid-array/discuss/2283929/Java-O(N)
/// Note:
/// the key point is the keep track of the
/// 1. the index of the smallest number, if there are more than one, the leftmost one,
/// 2. the index of the largest number, if there are more than one, the rightmost one
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_swaps(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let mut idx_max: usize = len_ns - 1;
        let mut idx_min: usize = 0;
        for idx in 1..len_ns {
            if nums[idx_max] < nums[len_ns - 1 - idx] {
                idx_max = len_ns - 1 - idx;
            }
            if nums[idx_min] > nums[idx] {
                idx_min = idx;
            }
        }
        (len_ns - 1 - idx_max + idx_min - if idx_min > idx_max { 1 } else { 0 }) as i32
    }
}
