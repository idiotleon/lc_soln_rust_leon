/// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/
///
/// Time Complexity:    O(lg(`len_n`))
/// Space Complexity:   O(1)
///
/// Reference:
/// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/discuss/28218/My-8ms-C++-solution-(o(logn)-on-average-o(n)-worst-case)/284690
/// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/discuss/28218/My-8ms-C%2B%2B-solution-(o(logn)-on-average-o(n)-worst-case)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let len_n: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_n - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if target == nums[mid] {
                return true;
            }
            if nums[lo] < nums[mid] {
                if nums[lo] <= target && target < nums[mid] {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else if nums[lo] == nums[mid] {
                lo += 1;
            } else {
                if nums[mid] < target && target <= nums[hi] {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
        }
        false
    }
}
