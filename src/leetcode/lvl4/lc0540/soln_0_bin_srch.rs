/// @author: Leon
/// https://leetcode.com/problems/single-element-in-a-sorted-array/
/// Time Complexity:    O(lg(`len_n`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/single-element-in-a-sorted-array/discuss/1587318/Java-Binary-Search-or-Beats-100-or-Most-Intutive-or-Explanation-Using-Image
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let len_n = nums.len();
        if len_n == 1 {
            return nums[0];
        }
        let mut lo = 0;
        let mut hi = len_n - 1;
        while lo < hi {
            let mid = {
                let mut mid = lo + (hi - lo) / 2;
                if nums[mid] == nums[1 + mid] {
                    mid = mid - 1;
                }
                mid
            };
            let len_left_half = mid - lo + 1;
            if len_left_half % 2 != 0 {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        nums[lo]
    }
}
