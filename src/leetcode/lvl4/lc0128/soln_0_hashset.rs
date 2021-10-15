use std::collections::HashSet;
/// https://leetcode.com/problems/longest-consecutive-sequence/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`_len_n`)
/// Reference:
/// https://leetcode.com/problems/longest-consecutive-sequence/discuss/728511/Rust
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let _len_n = nums.len();
        let set = nums.into_iter().collect::<HashSet<i32>>();
        let mut longest = 0;
        for num in &set {
            if set.contains(&(*num - 1)) {
                continue;
            }
            let mut cur = *num;
            let mut len = 1;
            while set.contains(&(cur + 1)) {
                cur += 1;
                len += 1;
            }
            longest = std::cmp::max(longest, len);
        }
        longest
    }
}
