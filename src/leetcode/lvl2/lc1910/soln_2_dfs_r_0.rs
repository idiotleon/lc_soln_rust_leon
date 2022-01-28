/// @author: Leon
/// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
/// Time Complexity:    O(`_len_s` ^ 2 * `len_p`)
/// Space Complexity:   O(`_len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        Self::dfs(s, part)
    }
    fn dfs(s: String, part: String) -> String {
        match s[..].find(&part) {
            Some(_) => Self::dfs(s.replacen(&part, "", 1), part),
            None => s,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let s = "daabcbaabcbc".to_owned();
        let part = "abc".to_owned();
        let actual = Solution::remove_occurrences(s, part);
        let expected = "dab".to_owned();
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let s = "axxxxyyyyb".to_owned();
        let part = "xy".to_owned();
        let actual = Solution::remove_occurrences(s, part);
        let expected = "ab".to_owned();
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_test_case_77() {
        let s = "aabababa".to_owned();
        let part = "aba".to_owned();
        let actual = Solution::remove_occurrences(s, part);
        let expected = "ba".to_owned();
        assert_eq!(expected, actual);
    }
}
