/// @author: Leon
/// https://leetcode.com/problems/is-subsequence/
/// Time Complexity:    O(`len_s` + `len_t`) ~ O(`len_t`)
/// Space Complexity:   O(`len_s` + `len_t`) ~ O(`len_t`)
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
        let chs_t: Vec<char> = t.chars().collect();
        let mut idx_s: usize = 0;
        let mut idx_t: usize = 0;
        while idx_t < len_t {
            if chs_s[idx_s] == chs_t[idx_t] {
                idx_s += 1;
                if idx_s == len_s {
                    return true;
                }
            }
            idx_t += 1;
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
