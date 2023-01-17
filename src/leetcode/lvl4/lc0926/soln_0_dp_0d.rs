/// @author: Leon
/// https://leetcode.com/problems/flip-string-to-monotone-increasing/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/flip-string-to-monotone-increasing/discuss/189751/C++-one-pass-DP-solution-0ms-O(n)-or-O(1)-one-line-with-explaination./318321
/// https://leetcode.com/problems/flip-string-to-monotone-increasing/discuss/189751/C%2B%2B-one-pass-DP-solution-0ms-O(n)-or-O(1)-one-line-with-explaination.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        const ONE: char = '1';
        const ZERO: char = '0';
        let mut cnt_ones: u16 = 0;
        let mut cnt_flips: u16 = 0;
        for ch in s.chars() {
            match ch {
                ZERO => cnt_flips += 1,
                ONE => cnt_ones += 1,
                _ => unreachable!(),
            }
            cnt_flips = std::cmp::min(cnt_flips, cnt_ones);
        }
        return cnt_flips as i32;
    }
}
