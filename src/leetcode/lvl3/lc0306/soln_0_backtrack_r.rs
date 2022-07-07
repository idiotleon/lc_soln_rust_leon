/// @author: Leon
/// https://leetcode.com/problems/additive-number/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/additive-number/discuss/75567/Java-Recursive-and-Iterative-Solutions
/// https://leetcode.com/problems/additive-number/discuss/139895/Logical-Thinking-with-Clear-Java-Code
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let len_s: usize = num.len();
        let chs: Vec<char> = num.chars().collect();
        let mut res: Vec<i64> = Vec::with_capacity(len_s);
        Self::dfs(0, &chs, &mut res)
    }
    fn dfs(idx_start: usize, chs: &Vec<char>, res: &mut Vec<i64>) -> bool {
        let len_cs: usize = chs.len();
        if idx_start == len_cs && res.len() >= 3 {
            return true;
        }
        let mut num: i64 = 0;
        for idx in idx_start..len_cs {
            if idx != idx_start && chs[idx_start] == '0' {
                break;
            }
            num = num * 10 + chs[idx] as i64 - '0' as i64;
            if res.len() < 2 || num == res[res.len() - 1] + res[res.len() - 2] {
                res.push(num);
                if Self::dfs(idx + 1, chs, res) {
                    return true;
                }
                res.pop();
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_test_case_41_should_return_expected() {
        let num = "11111111111011111111111".to_owned();
        let expected = true;
        let actual = Solution::is_additive_number(num);
        assert_eq!(expected, actual);
    }
}
