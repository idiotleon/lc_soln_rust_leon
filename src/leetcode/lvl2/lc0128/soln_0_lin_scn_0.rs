use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/longest-consecutive-sequence/
/// Time Complexity:    O(amortized(`_len_ns`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/longest-consecutive-sequence/discuss/41055/My-really-simple-Java-O(n)-solution-Accepted/191109
/// https://leetcode.com/problems/longest-consecutive-sequence/discuss/41055/My-really-simple-Java-O(n)-solution-Accepted
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        let mut seen: HashSet<i32> = nums.to_vec().into_iter().collect();
        let mut longest: i32 = 0;
        for &num in &nums {
            let mut lo: i32 = num - 1;
            let mut hi: i32 = num + 1;
            while seen.remove(&lo) {
                lo -= 1;
            }
            while seen.remove(&hi) {
                hi += 1;
            }
            longest = std::cmp::max(longest, hi - lo - 1);
            if seen.is_empty() {
                break;
            }
        }
        longest as i32
    }
}
