/// @author: Leon
/// https://leetcode.com/problems/get-equal-substrings-within-budget/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let len_s: usize = s.len();
        let chs_s: Vec<char> = s.chars().collect();
        let chs_t: Vec<char> = t.chars().collect();
        let mut cost: i32 = 0;
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        let mut longest: usize = 0;
        while hi < len_s {
            cost += (chs_t[hi] as i32 - chs_s[hi] as i32).abs();
            if cost <= max_cost {
                let len = hi - lo + 1;
                longest = std::cmp::max(longest, len);
            } else {
                while cost > max_cost {
                    cost -= (chs_t[lo] as i32 - chs_s[lo] as i32).abs();
                    lo += 1;
                }
            }
            hi += 1;
        }
        longest as i32
    }
}
