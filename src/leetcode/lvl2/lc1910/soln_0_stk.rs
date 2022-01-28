/// @author: Leon
/// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
/// Time Complexity:    O(`_len_s` * `len_s`)
/// Space Complexity:   O(`_len_s`)
/// Reference:
/// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/discuss/1298766/C%2B%2B-Simple-solution-Faster-than-100
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let _len_s: usize = s.len();
        let len_p: usize = part.len();
        let mut chs: Vec<char> = (&s).chars().collect();
        let chs_p: Vec<char> = part.chars().collect();
        let mut idx: usize = 0;
        for ch in (&s).chars() {
            chs[idx] = ch;
            idx += 1;
            if idx >= len_p && Self::is_substr(idx - len_p, &chs, &chs_p) {
                idx -= len_p
            }
        }
        (&chs[..idx]).into_iter().collect::<String>()
    }
    fn is_substr(idx_start: usize, chs: &Vec<char>, chs_p: &Vec<char>) -> bool {
        let len_s: usize = chs.len();
        let len_p: usize = chs_p.len();
        let mut idx: usize = idx_start;
        let mut idx_p: usize = 0;
        while idx < len_s && idx_p < len_p {
            if chs[idx] != chs_p[idx_p] {
                return false;
            }
            idx += 1;
            idx_p += 1;
        }
        true
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
