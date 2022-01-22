/// @author: Leon
/// https://leetcode.com/problems/backspace-string-compare/
/// Time Complexity:    O(`len_s` + `len_t`)
/// Space Complexity:   O(1) / O(`len_s` + `len_t`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        const BACKSPACE: char = '#';
        const IMPL: char = '$';
        let len_s: usize = s.len();
        let len_t: usize = t.len();
        let chs_s: Vec<char> = s.chars().collect();
        let chs_t: Vec<char> = t.chars().collect();
        let mut idx_s: isize = len_s as isize - 1;
        let mut idx_t: isize = len_t as isize - 1;
        let mut skip_s: u8 = 0;
        let mut skip_t: u8 = 0;
        while idx_s >= 0 || idx_t >= 0 {
            let ch_s: char = {
                while idx_s >= 0 && (skip_s > 0 || chs_s[idx_s as usize] == BACKSPACE) {
                    match chs_s[idx_s as usize] {
                        BACKSPACE => skip_s += 1,
                        _ => skip_s -= 1,
                    }
                    idx_s -= 1;
                }
                if idx_s >= 0 {
                    chs_s[idx_s as usize]
                } else {
                    IMPL
                }
            };
            let ch_t: char = {
                while idx_t >= 0 && (skip_t > 0 || chs_t[idx_t as usize] == BACKSPACE) {
                    match chs_t[idx_t as usize] {
                        BACKSPACE => skip_t += 1,
                        _ => skip_t -= 1,
                    }
                    idx_t -= 1;
                }
                if idx_t >= 0 {
                    chs_t[idx_t as usize]
                } else {
                    IMPL
                }
            };
            if ch_s != ch_t {
                return false;
            }
            idx_s -= 1;
            idx_t -= 1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let s: String = "ab#c".to_owned();
        let t: String = "ad#c".to_owned();
        let actual = Solution::backspace_compare(s, t);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let s: String = "ab##".to_owned();
        let t: String = "a#d#".to_owned();
        let actual = Solution::backspace_compare(s, t);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let s: String = "a#c".to_owned();
        let t: String = "b".to_owned();
        let actual = Solution::backspace_compare(s, t);
        let expected = false;
        assert_eq!(expected, actual);
    }
}
