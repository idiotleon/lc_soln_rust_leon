use std::collections::HashSet;
/// @author: Leon
/// https://leetcode.com/problems/repeated-dna-sequences/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let len_s: usize = s.len();
        let mut seen: HashSet<&str> = HashSet::new();
        let mut ans: HashSet<&str> = HashSet::new();
        let mut hi: usize = 10;
        let mut lo: usize = 0;
        while hi <= len_s {
            let sub = &s[lo..hi];
            if !seen.insert(sub) {
                ans.insert(sub);
            }
            hi += 1;
            lo += 1;
        }
        ans.into_iter().map(|e| e.to_owned()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_sample_input_2_should_return_expected() {
        let s: String = "AAAAAAAAAAAAA".to_owned();
        let actual: Vec<String> = Solution::find_repeated_dna_sequences(s);
        let expected: Vec<String> = vec!["AAAAAAAAAA".to_owned()];
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn test_test_case_25_should_return_expected() {
        let s: String = "AAAAAAAAAAA".to_owned();
        let actual: Vec<String> = Solution::find_repeated_dna_sequences(s);
        let expected: Vec<String> = vec!["AAAAAAAAAA".to_owned()];
        assert_eq!(expected, actual);
    }
}
