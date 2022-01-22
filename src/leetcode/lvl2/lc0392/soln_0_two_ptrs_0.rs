/// @author: Leon
/// https://leetcode.com/problems/is-subsequence/
/// Time Complexity:    O(`len_s` + `len_t`) ~ O(`len_t`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let len_s: usize = s.len();
        let len_t: usize = t.len();
        if len_s > len_t {
            return false;
        }
        let chs_s: Vec<char> = s.chars().collect();
        let mut idx_s: usize = 0;
        for ch in t.chars() {
            if chs_s[idx_s] == ch {
                idx_s += 1;
                if idx_s == len_s {
                    return true;
                }
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
        let s = "abc".to_owned();
        let t = "ahbgdc".to_owned();
        let actual = Solution::is_subsequence(s, t);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
