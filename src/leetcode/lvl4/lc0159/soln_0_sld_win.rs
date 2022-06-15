/// @author: Leon
/// https://leetcode.com/problems/longest-substring-with-at-most-two-distinct-characters/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut freqs: Vec<u16> = vec![0; 128];
        let mut cnt: u16 = 0;
        const THRESHOLD: u16 = 2;
        let mut hi: usize = 0;
        let mut lo: usize = 0;
        let mut longest: usize = 0;
        while hi < len_s {
            let idx_ch_hi: usize = chs[hi] as usize;
            if freqs[idx_ch_hi] == 0 {
                cnt += 1;
            }
            freqs[idx_ch_hi] += 1;
            if cnt > THRESHOLD {
                let idx_ch_lo: usize = chs[lo] as usize;
                freqs[idx_ch_lo] -= 1;
                if freqs[idx_ch_lo] == 0 {
                    cnt -= 1;
                }
                lo += 1;
            }
            let len = hi - lo + 1;
            longest = std::cmp::max(longest, len);
            hi += 1;
        }
        longest as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_sample_input_1_should_return_expected() {
        let s: String = "eceba".to_owned();
        let actual = Solution::length_of_longest_substring_two_distinct(s);
        let expected = 3;
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn test_sample_input_2_should_return_expected() {
        let s: String = "ccaabbb".to_owned();
        let actual = Solution::length_of_longest_substring_two_distinct(s);
        let expected = 5;
        assert_eq!(expected, actual);
    }
}
