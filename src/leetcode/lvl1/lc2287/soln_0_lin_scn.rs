/// @author: Leon
/// https://leetcode.com/problems/rearrange-characters-to-make-target-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let len_s: usize = s.len();
        let freqs_s: Vec<u8> = {
            let mut freqs: Vec<u8> = vec![0; 26];
            for &b in s.as_bytes() {
                freqs[(b - b'a') as usize] += 1;
            }
            freqs
        };
        let freqs_t: Vec<u8> = {
            let mut freqs: Vec<u8> = vec![0; 26];
            for &b in target.as_bytes() {
                freqs[(b - b'a') as usize] += 1;
            }
            freqs
        };
        let mut fewest: u8 = len_s as u8;
        for idx in 0..26 {
            if freqs_t[idx] == 0 {
                continue;
            }
            fewest = std::cmp::min(fewest, freqs_s[idx] / freqs_t[idx]);
        }
        fewest as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let s: String = "ilovecodingonleetcode".to_owned();
        let target: String = "code".to_owned();
        let expected: i32 = 2;
        let actual: i32 = Solution::rearrange_characters(s, target);
        assert_eq!(expected, actual);
    }
}
