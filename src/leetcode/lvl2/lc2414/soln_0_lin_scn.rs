/// @author: Leon
/// https://leetcode.com/problems/length-of-the-longest-alphabetical-continuous-substring/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let _len_s: usize = s.len();
        let mut prev: u8 = '#' as u8;
        let mut len: u16 = 1;
        let mut longest: u16 = 1;
        for (idx, ch) in s.chars().enumerate() {
            if idx == 0 {
                prev = ch as u8;
                continue;
            }
            if ch as u8 == prev as u8 + 1 {
                len += 1;
                longest = std::cmp::max(longest, len);
            } else {
                len = 1;
            }
            prev = ch as u8;
        }
        return longest as i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_work() {
        let s: String = "abacaba".to_owned();
        let expected: i32 = 2;
        let actual: i32 = Solution::longest_continuous_substring(s);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_test_case_16_should_work() {
        let s: String = "yrpjofyzubfsiypfre".to_owned();
        let expected: i32 = 2;
        let actual: i32 = Solution::longest_continuous_substring(s);
        assert_eq!(expected, actual);
    }
}
