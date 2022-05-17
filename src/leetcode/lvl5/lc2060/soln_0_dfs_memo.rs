/// @author: Leon
/// https://leetcode.com/problems/check-if-an-original-string-exists-given-two-encoded-strings/
/// Time Complexity:    O(`len1` * `len_2`)
/// Space Complexity:   O(`len1` * `len_2`)
/// Reference:
/// https://leetcode.com/problems/check-if-an-original-string-exists-given-two-encoded-strings/discuss/1550342/Java-Clean-(DFS-+-memo)/1134399
/// https://leetcode.com/problems/check-if-an-original-string-exists-given-two-encoded-strings/discuss/1550342/Java-Clean-(DFS-%2B-memo)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn possibly_equals(s1: String, s2: String) -> bool {
        let len1: usize = s1.len();
        let chs1: Vec<char> = s1.chars().collect();
        let len2: usize = s2.len();
        let chs2: Vec<char> = s2.chars().collect();
        let mut memo: Vec<Vec<Vec<Option<bool>>>> =
            vec![vec![vec![None; 2000]; len2 + 1]; len1 + 1];
        Self::dfs(0, 0, 0, &chs1, &chs2, &mut memo)
    }
    fn dfs(
        idx1: usize,
        idx2: usize,
        diff: i16,
        chs1: &Vec<char>,
        chs2: &Vec<char>,
        memo: &mut Vec<Vec<Vec<Option<bool>>>>,
    ) -> bool {
        const RADIX: u32 = 10;
        let len1: usize = chs1.len();
        let len2: usize = chs2.len();
        if idx1 == len1 && idx2 == len2 {
            return diff == 0;
        }
        if let Some(m) = memo[idx1][idx2][(diff + 1000) as usize] {
            return m;
        }
        // literal matching for `chs1[idx1]` and `chs2[idx2]`
        if idx1 < len1 && idx2 < len2 && diff == 0 && chs1[idx1] == chs2[idx2] {
            if Self::dfs(idx1 + 1, idx2 + 1, 0, chs1, chs2, memo) {
                memo[idx1][idx2][1000] = Some(true);
                return true;
            }
        }
        // literal matching for `chs1[idx1]`
        if idx1 < len1
            && !chs1[idx1].is_digit(RADIX)
            && diff > 0
            && Self::dfs(idx1 + 1, idx2, diff - 1, chs1, chs2, memo)
        {
            memo[idx1][idx2][(diff + 1000) as usize] = Some(true);
            return true;
        }
        // literal matching for `chs2[idx2]`
        if idx2 < len2
            && !chs2[idx2].is_digit(RADIX)
            && diff < 0
            && Self::dfs(idx1, idx2 + 1, diff + 1, chs1, chs2, memo)
        {
            memo[idx1][idx2][(diff + 1000) as usize] = Some(true);
            return true;
        }
        // wild card matching on `chs1[idx1]`
        let mut num1: i16 = 0;
        for idx in idx1..len1 {
            if !chs1[idx].is_digit(RADIX) {
                break;
            }
            num1 = (num1 * 10) + (chs1[idx] as i16 - '0' as i16);
            if Self::dfs(idx + 1, idx2, diff - num1, chs1, chs2, memo) {
                memo[idx1][idx2][(diff + 1000) as usize] = Some(true);
                return true;
            }
        }
        // wild card matching for `chs2[idx2]`
        let mut num2: i16 = 0;
        for idx in idx2..len2 {
            if !chs2[idx].is_digit(RADIX) {
                break;
            }
            num2 = (num2 * 10) + (chs2[idx] as i16 - '0' as i16);
            if Self::dfs(idx1, idx + 1, diff + num2, chs1, chs2, memo) {
                memo[idx1][idx2][(diff + 1000) as usize] = Some(true);
                return true;
            }
        }
        memo[idx1][idx2][(diff + 1000) as usize] = Some(false);
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_case_20_should_return_expected() {
        let s1: String = "112s".to_owned();
        let s2: String = "g841".to_owned();
        let actual = Solution::possibly_equals(s1, s2);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
