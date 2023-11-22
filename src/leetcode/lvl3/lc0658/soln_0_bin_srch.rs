/// @author: Leon
/// https://leetcode.com/problems/find-k-closest-elements/
/// Time Complexity:    O(lg(`len_ns` - `k`) + `k`)
/// Space Complexity:   O(1) / O(`k`)
/// Reference:
/// https://leetcode.com/problems/find-k-closest-elements/discuss/106426/JavaC%2B%2BPython-Binary-Search-O(log(N-K)-%2B-K)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_closest_elements(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let k: usize = k as usize;
        let mut lo: usize = 0;
        let mut hi: usize = len_ns - k;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if x - nums[mid] > nums[mid + k] - x {
                lo = 1 + mid;
            } else {
                hi = mid;
            }
        }
        return nums[lo..lo + k].to_vec();
    }
}
