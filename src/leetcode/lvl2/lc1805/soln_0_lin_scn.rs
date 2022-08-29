use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/number-of-different-integers-in-a-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let len_s: usize = word.len();
        let chs: Vec<char> = word.chars().collect();
        let mut seen: HashSet<String> = HashSet::with_capacity(len_s);
        let mut lo: usize = 0;
        while lo < len_s {
            if Self::is_digit(chs[lo]) {
                let (mid, hi): (usize, usize) = {
                    let mut mid: usize = len_s;
                    let mut hi: usize = lo;
                    while hi < len_s && Self::is_digit(chs[hi]) {
                        if mid == len_s && chs[hi] != '0' {
                            mid = hi;
                        }
                        hi += 1;
                    }
                    (mid, hi)
                };
                if mid == len_s {
                    seen.insert("0".to_owned());
                } else {
                    let substr: String = Self::get_substr(mid, hi, &chs);
                    seen.insert(substr);
                }
                lo = hi;
            } else {
                lo += 1;
            }
        }
        return seen.len() as i32;
    }
    fn get_substr(lo: usize, hi: usize, chs: &Vec<char>) -> String {
        let sub: &mut String = &mut "".to_owned();
        for idx in lo..hi {
            sub.push(chs[idx]);
        }
        return sub.to_owned();
    }
    fn is_digit(ch: char) -> bool {
        return ch >= '0' && ch <= '9';
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let word: String = "a123bc34d8ef34".to_owned();
        let expected: i32 = 3;
        let actual = Solution::num_different_integers(word);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let word: String = "leet1234code234".to_owned();
        let expected: i32 = 2;
        let actual = Solution::num_different_integers(word);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_3_should_return_expected() {
        let word: String = "a1b01c001".to_owned();
        let expected: i32 = 1;
        let actual = Solution::num_different_integers(word);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_test_case_64_should_return_expected() {
        let word: String = "0a0".to_owned();
        let expected: i32 = 1;
        let actual = Solution::num_different_integers(word);
        assert_eq!(expected, actual);
    }
}
