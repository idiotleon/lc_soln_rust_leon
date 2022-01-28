/// @author: Leon
/// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
/// Time Complexity:    O(`_len_s` ^ 2 * `len_p`)
/// Space Complexity:   O(`_len_s`)
/// !!!TLEed!!!
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let _len_s: usize = s.len();
        Self::dfs(&s, &part.chars().collect())
    }
    fn dfs(s: &str, chs_p: &Vec<char>) -> String {
        let len_s: usize = s.len();
        let len_p: usize = chs_p.len();
        let chs_s: Vec<char> = s.chars().collect();
        let mut ans: String = s.to_owned();
        for (idx, ch) in s.chars().enumerate() {
            if ch == chs_p[0] {
                let mut cnt: usize = 0;
                let mut idx_s: usize = idx;
                let mut idx_p: usize = 0;
                while idx_p < len_p && idx_s < len_s && chs_s[idx_s] == chs_p[idx_p] {
                    idx_s += 1;
                    idx_p += 1;
                    cnt += 1;
                }
                if cnt == len_p {
                    let mut res = s[..idx].to_owned();
                    res.push_str(&s[idx_s..]);
                    ans = Self::dfs(&res, chs_p);
                }
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
}
