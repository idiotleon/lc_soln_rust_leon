/// @author: Leon
/// https://leetcode.com/problems/find-peak-element/
/// Time Complexity:    O(lg(`len_n`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let len_n = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_n - 1;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] < nums[mid + 1] {
                lo = 1 + mid;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
}
