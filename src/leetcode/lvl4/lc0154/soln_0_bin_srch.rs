/// @author: Leon
/// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/discuss/1339694/Rust-with-comments
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let len_n = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_n - 1;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] < nums[hi] {
                hi = mid;
            } else if nums[mid] > nums[hi] {
                lo = mid + 1;
            } else {
                hi -= 1;
            }
        }
        nums[lo]
    }
}
