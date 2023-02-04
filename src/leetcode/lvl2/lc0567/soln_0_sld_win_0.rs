/// @author: Leon
/// https://leetcode.com/problems/permutation-in-string/
/// Time Complexity:    O(`len2`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let len1: usize = s1.len();
        let len2: usize = s2.len();
        let chs: Vec<char> = s2.chars().collect();
        let mut cnt: usize = len1;
        let mut freqs: Vec<i32> = {
            let mut freqs: Vec<i32> = vec![0; 26];
            for ch in s1.chars() {
                freqs[ch as usize - 'a' as usize] += 1;
            }
            freqs
        };
        let mut hi: usize = 0;
        let mut lo: usize = 0;
        while hi < len2 {
            let freq_hi = &mut freqs[chs[hi] as usize - 'a' as usize];
            if *freq_hi > 0 {
                cnt -= 1;
            }
            *freq_hi -= 1;
            if cnt == 0 {
                return true;
            }
            if hi + 1 >= len1 {
                let freq_lo = &mut freqs[chs[lo] as usize - 'a' as usize];
                *freq_lo += 1;
                if *freq_lo > 0 {
                    cnt += 1;
                }
                lo += 1;
            }
            hi += 1;
        }
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_sample_input_1_should_return_expected() {
        let s1: String = "ab".to_owned();
        let s2: String = "eidbaooo".to_owned();
        let actual = Solution::check_inclusion(s1, s2);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn test_test_input_92_should_return_expected() {
        let s1: String = "ab".to_owned();
        let s2: String = "eidboaoo".to_owned();
        let actual = Solution::check_inclusion(s1, s2);
        let expected = false;
        assert_eq!(expected, actual);
    }
}
