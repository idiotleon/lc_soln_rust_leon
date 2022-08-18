/// @author: Leon
/// https://leetcode.com/problems/check-if-numbers-are-ascending-in-a-sentence/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let _len_s: usize = s.len();
        let mut prev: u32 = 0;
        let mut num: u32 = 0;
        let mut should_cal: bool = false;
        for ch in s.chars() {
            if ch >= '0' && ch <= '9' {
                num = num * 10 + (ch as u32 - '0' as u32);
                should_cal = true;
            } else {
                if should_cal {
                    if prev >= num {
                        return false;
                    }
                    prev = num;
                    num = 0;
                }
                should_cal = false;
            }
        }
        // for the trailing digit(s)
        if num > 0 {
            return prev < num;
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let s: String = "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_owned();
        let expected: bool = true;
        let actual = Solution::are_numbers_ascending(s);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let s: String = "hello world 5 x 5".to_owned();
        let expected: bool = false;
        let actual = Solution::are_numbers_ascending(s);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_3_should_return_expected() {
        let s: String =
            "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_owned();
        let expected: bool = false;
        let actual = Solution::are_numbers_ascending(s);
        assert_eq!(expected, actual);
    }
}
