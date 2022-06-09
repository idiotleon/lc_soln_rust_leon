/// @author: Leon
/// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
/// Time Complexity:    O(`len_n` * lg(`len_n`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len_n: usize = nums.len();
        let mut idx: usize = 0;
        while idx < len_n {
            while idx < len_n && idx + 2 < len_n && nums[idx] == nums[idx + 2] {
                idx += 1;
            }
            if nums[idx] > target / 2 {
                break;
            }
            let idx_exp: i32 = Self::binary_search(idx + 1, target - nums[idx], &nums);
            if idx_exp > 0 {
                return vec![idx as i32 + 1, idx_exp];
            }
            idx += 1;
        }
        unreachable!()
    }
    fn binary_search(idx_start: usize, target: i32, nums: &Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let mut lo: usize = idx_start;
        let mut hi: usize = len_n - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] == target {
                return mid as i32 + 1;
            }
            if nums[mid] > target {
                hi -= 1;
            } else {
                lo += 1;
            }
        }
        -1
    }
}
