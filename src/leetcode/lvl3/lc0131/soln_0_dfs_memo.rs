/// https://leetcode.com/problems/palindrome-partitioning/
/// Time Complexity:    O(`len_s` * (2 ^ `len_s`))
/// Space Complexity:   O(`len_s` ^ 2) + O(`len_s`)
/// https://leetcode.com/problems/palindrome-partitioning/discuss/41982/Java-DP-%2B-DFS-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let len_s: usize = s.len();
        let mut path: Vec<String> = Vec::new();
        let mut paths: Vec<Vec<String>> = Vec::new();
        let is_palindrome: Vec<Vec<bool>> = {
            let mut is_palindrome: Vec<Vec<bool>> = vec![vec![false; len_s]; len_s];
            let chs: Vec<char> = s.chars().collect();
            for hi in 0..len_s {
                for lo in 0..=hi {
                    if chs[lo] == chs[hi] && (hi - lo <= 2 || is_palindrome[lo + 1][hi - 1]) {
                        is_palindrome[lo][hi] = true;
                    }
                }
            }
            is_palindrome
        };
        Self::backtrack(0, &mut path, &s, &is_palindrome, &mut paths);
        paths
    }
    fn backtrack(
        idx_start: usize,
        path: &mut Vec<String>,
        s: &str,
        is_palindrome: &Vec<Vec<bool>>,
        paths: &mut Vec<Vec<String>>,
    ) {
        let len_s: usize = s.len();
        if idx_start == len_s {
            paths.push(path.to_vec());
            return;
        }
        for idx in idx_start..len_s {
            if is_palindrome[idx_start][idx] {
                path.push(s[idx_start..idx + 1].to_owned());
                Self::backtrack(idx + 1, path, s, is_palindrome, paths);
                path.pop();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let s = "aab".to_owned();
        let actual = {
            let mut sorted = Solution::partition(s);
            sorted.sort();
            sorted
        };
        let expected = {
            let mut sorted = vec![
                vec!["a".to_owned(), "a".to_owned(), "b".to_owned()],
                vec!["aa".to_owned(), "b".to_owned()],
            ];
            sorted.sort();
            sorted
        };
        assert_eq!(expected, actual);
    }
}
