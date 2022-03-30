/// @author: Leon
/// https://leetcode.com/problems/missing-element-in-sorted-array/
/// Time Complexity:    O(lg(`len_n`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/missing-element-in-sorted-array/discuss/303444/Java-O(logN)-solution-Binary-Search/830433
/// https://leetcode.com/problems/missing-element-in-sorted-array/discuss/303444/Java-O(logN)-solution-Binary-Search
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn missing_element(nums: Vec<i32>, k: i32) -> i32 {
        let len_n: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_n - 1;
        let mut cnt_missing: i32 = 0;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            cnt_missing = nums[mid] - nums[0] - mid as i32;
            if k > cnt_missing {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        if cnt_missing >= k {
            return nums[lo] - 1 - (cnt_missing - k);
        }
        return nums[hi] + (k - cnt_missing);
    }
}
