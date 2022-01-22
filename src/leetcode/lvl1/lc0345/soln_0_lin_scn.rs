/// @author: Leon
/// https://leetcode.com/problems/reverse-vowels-of-a-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let len_s: usize = s.len();
        let mut chs: Vec<char> = s.chars().collect();
        let mut lo: isize = 0;
        let mut hi: isize = len_s as isize - 1;
        while lo < hi {
            while lo < hi && !Self::is_vowel(chs[lo as usize]) {
                lo += 1;
            }
            while lo < hi && !Self::is_vowel(chs[hi as usize]) {
                hi -= 1;
            }
            chs.swap(lo as usize, hi as usize);
            lo += 1;
            hi -= 1;
        }
        chs.into_iter().collect()
    }
    fn is_vowel(ch: char) -> bool {
        const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        for &vowel in VOWELS {
            if ch == vowel {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let s = "hello".to_owned();
        let actual = Solution::reverse_vowels(s);
        let expected = "holle".to_owned();
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_4() {
        let s = "a.".to_owned();
        let actual = Solution::reverse_vowels(s);
        let expected = "a.".to_owned();
        assert_eq!(expected, actual);
    }
}
