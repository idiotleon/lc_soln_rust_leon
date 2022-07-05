use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/longest-consecutive-sequence/
/// Time Complexity:    O(amortized(`_len_ns`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/longest-consecutive-sequence/discuss/41057/Simple-O(n)-with-Explanation-Just-walk-each-streak/39195
/// https://leetcode.com/problems/longest-consecutive-sequence/discuss/41057/Simple-O(n)-with-Explanation-Just-walk-each-streak
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        let seen: HashSet<i32> = nums.into_iter().collect();
        let mut longest: i32 = 0;
        for &lo in &seen {
            if !seen.contains(&(lo - 1)) {
                let mut hi = lo + 1;
                while seen.contains(&hi) {
                    hi += 1;
                }
                longest = std::cmp::max(longest, hi - lo);
            }
        }
        longest as i32
    }
}
