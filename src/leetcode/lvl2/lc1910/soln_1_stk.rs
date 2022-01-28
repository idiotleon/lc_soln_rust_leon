/// @author: Leon
/// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
/// Time Complexity:    O(`_len_s` * `len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let _len_s: usize = s.len();
        let len_p: usize = part.len();
        let mut ans: String = "".to_owned();
        for ch in s.chars() {
            ans.push(ch);
            if ans.len() >= len_p && (&ans[ans.len() - len_p..]).to_owned() == part {
                ans = ans.replace(&part, "");
            }
        }
        ans
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
