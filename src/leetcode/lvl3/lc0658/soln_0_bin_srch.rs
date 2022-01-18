/// @author: Leon
/// https://leetcode.com/problems/find-k-closest-elements/
/// Time Complexity:    O(lg(`len_n` - `k`) + `k`)
/// Space Complexity:   O(1) / O(`k`)
/// Reference:
/// https://leetcode.com/problems/find-k-closest-elements/discuss/106426/JavaC%2B%2BPython-Binary-Search-O(log(N-K)-%2B-K)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_closest_elements(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let len_n: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_n - k as usize;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if x - nums[mid] > nums[mid + k as usize] - x {
                lo = 1 + mid;
            } else {
                hi = mid;
            }
        }
        nums[lo..lo + k as usize].to_vec()
    }
}
