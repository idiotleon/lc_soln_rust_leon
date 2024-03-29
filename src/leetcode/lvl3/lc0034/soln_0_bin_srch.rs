/// @author: Leon
/// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
/// Time Complexity:     O(`_n_len`)
/// Space Complexity:    O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let _len_n: usize = nums.len();
        let first_pos = Self::lower_bound(&nums, target);
        let last_pos = Self::upper_bound(&nums, target);
        if first_pos > last_pos {
            return vec![-1, -1];
        }
        vec![first_pos, last_pos]
    }
    fn lower_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let len: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len;
        while lo < hi {
            let mid: usize = lo + (hi - lo) / 2;
            if target > nums[mid] {
                lo = 1 + mid;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
    fn upper_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let len: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len;
        while lo < hi {
            let mid: usize = lo + (hi - lo) / 2;
            if target < nums[mid] {
                hi = mid;
            } else {
                lo = 1 + mid;
            }
        }
        hi as i32 - 1
    }
}
