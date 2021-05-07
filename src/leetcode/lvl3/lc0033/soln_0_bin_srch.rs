/// https://leetcode.com/problems/search-in-rotated-sorted-array/
///
/// Time Compleixty:     O()
/// Space Complexity:    O()
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo: usize = 0;
        let mut hi: usize = nums.len() - 1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            if nums[mid] == target {
                return mid as i32;
            }

            if nums[lo] <= nums[mid] {
                if nums[lo] <= target && target < nums[mid] {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else if nums[mid] < nums[hi] {
                if nums[mid] < target && target <= nums[hi] {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
        }
        -1
    }
}
