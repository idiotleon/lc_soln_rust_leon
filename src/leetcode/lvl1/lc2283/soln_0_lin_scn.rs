/// @author: Leon
/// https://leetcode.com/problems/check-if-number-has-equal-digit-count-and-digit-value/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn digit_count(num: String) -> bool {
        let _len_s: usize = num.len();
        let freqs: Vec<u8> = {
            let mut freqs: Vec<u8> = vec![0; 10];
            for &b in num.as_bytes() {
                freqs[(b - b'0') as usize] += 1;
            }
            freqs
        };
        for (idx, &b) in num.as_bytes().into_iter().enumerate() {
            if freqs[idx] != (b - b'0') {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let num: String = "1210".to_owned();
        let expected = true;
        let actual = Solution::digit_count(num);
        assert_eq!(expected, actual);
    }
}
