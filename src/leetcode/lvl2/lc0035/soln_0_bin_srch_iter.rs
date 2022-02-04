/// @author: Leon
/// https://leetcode.com/problems/search-insert-position/
/// Time Complexity:    O(lg(`len_n`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/search-insert-position/discuss/1596603/C%2B%2BPython-Simple-Solutions-w-Explanation-or-Recursive-%2B-Iterative-Binary-Search-and-Built-in
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len_n = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
}
