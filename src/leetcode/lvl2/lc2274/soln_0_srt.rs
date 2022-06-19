/// @author: Leon
/// https://leetcode.com/problems/maximum-consecutive-floors-without-special-floors/
/// Time Complexity:    O(`len_fls` * lg(`len_fls`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        let len_fls: usize = special.len();
        let sorted: Vec<i32> = {
            let mut special = special;
            special.sort();
            special
        };
        let mut longest: i32 = 0;
        for (idx, &floor) in sorted.iter().enumerate() {
            if idx == 0 {
                longest = std::cmp::max(longest, floor - bottom);
            } else {
                longest = std::cmp::max(longest, sorted[idx] - sorted[idx - 1] - 1);
            }
        }
        longest = std::cmp::max(longest, top - sorted[len_fls - 1]);
        longest
    }
}
