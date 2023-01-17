/// @author: Leon
/// https://leetcode.com/problems/flip-string-to-monotone-increasing/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
/// Reference:
/// https://zxi.mytechroad.com/blog/dynamic-programming/leetcode-926-flip-string-to-monotone-increasing/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        const ONE: char = '1';
        const ZERO: char = '0';
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let prefix_ones: Vec<i32> = {
            let mut prefix: Vec<i32> = vec![0; len_s + 1];
            prefix[0] = chs[0] as i32 - ZERO as i32;
            for idx in 1..len_s {
                prefix[idx] = prefix[idx - 1] + (chs[idx] as i32 - ZERO as i32);
            }
            prefix
        };
        let suffix_zeros: Vec<i32> = {
            let mut suffix: Vec<i32> = vec![0; len_s + 1];
            suffix[len_s - 1] = ONE as i32 - chs[len_s - 1] as i32;
            for idx in (0..len_s - 1).rev() {
                suffix[idx] = suffix[idx + 1] + (ONE as i32 - chs[idx] as i32);
            }
            suffix
        };
        let mut min_flips = suffix_zeros[0];
        for idx in 1..=len_s {
            min_flips = std::cmp::min(min_flips, prefix_ones[idx - 1] + suffix_zeros[idx]);
        }
        return min_flips;
    }
}
