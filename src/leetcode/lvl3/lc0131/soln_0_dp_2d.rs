/// https://leetcode.com/problems/palindrome-partitioning/
/// Time Complexity:    O(`len_s` * (2 ^ `len_s`))
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/palindrome-partitioning/discuss/41974/My-Java-DP-only-solution-without-recursion.-O(n2)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut ans: Vec<Vec<Vec<String>>> = vec![vec![vec![]]; len_s + 1];
        let mut is_palindrome: Vec<Vec<bool>> = vec![vec![false; len_s]; len_s];
        for hi in 0..len_s {
            ans[hi + 1].clear();
            for lo in 0..=hi {
                if chs[lo] == chs[hi] && (hi - lo <= 2 || is_palindrome[lo + 1][hi - 1]) {
                    is_palindrome[lo][hi] = true;
                    let sub_str = &s[lo..hi + 1];
                    let len: usize = ans[lo].len();
                    for idx in 0..len {
                        let mut copy = ans[lo][idx].to_vec();
                        copy.push(sub_str.to_owned());
                        ans[hi + 1].push(copy);
                    }
                }
            }
        }
        ans[len_s].to_vec()
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
