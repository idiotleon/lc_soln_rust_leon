/// @author: Leon
/// https://leetcode.com/problems/backspace-string-compare/
/// Time Complexity:    O(`len_s` + `len_t`)
/// Space Complexity:   O(1) / O(`len_s` + `len_t`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::get_str(s) == Self::get_str(t)
    }
    fn get_str(s: String) -> String {
        const BACKSPACE: char = '#';
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut ans: String = "".to_owned();
        let mut skip: i32 = 0;
        for idx in (0..len_s).rev() {
            match chs[idx] {
                BACKSPACE => {
                    skip += 1;
                }
                _ => {
                    if skip > 0 {
                        skip -= 1;
                    } else {
                        ans.push(chs[idx]);
                    }
                }
            }
        }
        ans.chars().into_iter().rev().collect()
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
