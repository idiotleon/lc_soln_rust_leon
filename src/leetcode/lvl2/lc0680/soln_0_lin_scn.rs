/// @author: Leon
/// https://leetcode.com/problems/valid-palindrome-ii/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut lo: isize = 0;
        let mut hi: isize = len_s as isize - 1;
        while lo < hi {
            if chs[lo as usize] != chs[hi as usize] {
                return Self::is_palindrome(lo + 1, hi, &chs) || Self::is_palindrome(lo, hi - 1, &chs);
            }
            lo += 1;
            hi -= 1;
        }
        true
    }
    fn is_palindrome(lo: isize, hi: isize, chs: &Vec<char>) -> bool {
        let mut lo: isize = lo;
        let mut hi: isize = hi;
        while lo < hi {
            if chs[lo as usize] != chs[hi as usize] {
                return false;
            }
            lo += 1;
            hi -= 1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn it_works_with_sample_input_2() {
        let s: String = "abca".to_owned();
        let actual = Solution::valid_palindrome(s);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
