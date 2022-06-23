/// @author: Leon
/// https://leetcode.com/problems/find-peak-element/
/// Time Complexity:    O(lg(`len_n`))
/// Space Complexity:   O(lg(`len_n`))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let len_n = nums.len();
        Self::search(0, len_n - 1, &nums) as i32
    }
    fn search(lo: usize, hi: usize, nums: &Vec<i32>) -> usize {
        if lo == hi {
            return lo;
        }
        let mid = lo + (hi - lo) / 2;
        if nums[mid] > nums[mid + 1] {
            return Self::search(lo, mid, nums);
        } else {
            return Self::search(mid + 1, hi, nums);
        }
    }
}
