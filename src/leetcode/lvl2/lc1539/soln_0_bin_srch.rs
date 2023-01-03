/// @author: Leon
/// https://leetcode.com/problems/kth-missing-positive-number/
/// Time Complexity:    O(lg(`len_ns`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_kth_positive(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_ns;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] - mid as i32 - 1 >= k {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        return lo as i32 + k;
    }
}
