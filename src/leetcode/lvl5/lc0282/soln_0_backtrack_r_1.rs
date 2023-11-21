/// @author: Leon
/// https://leetcode.com/problems/expression-add-operators/
/// Time Complexity:    O((3 + 1) ^ `_len_s`)
/// Space Complexity:   O(`_len_s`)
/// Reference:
/// https://leetcode.com/problems/expression-add-operators/discuss/71895/Java-Standard-Backtrace-AC-Solutoin-short-and-clear
struct Solution;

#[allow(dead_code)]
impl Solution {
    const SIGN_PLUS: char = '+';
    const SIGN_MINUS: char = '-';
    const SIGN_MULTI: char = '*';
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let _len_s: usize = num.len();
        let mut ans: Vec<String> = Vec::new();
        let res: String = "".to_owned();
        let chs: Vec<char> = num.chars().collect();
        Self::backtrack(res, 0, 0, 0, &chs, target as i64, &mut ans);
        return ans;
    }
    fn backtrack(
        cur_res: String,
        prev: i64,
        multi: i64,
        idx_start: usize,
        chs: &Vec<char>,
        target: i64,
        res: &mut Vec<String>,
    ) {
        let len_cs: usize = chs.len();
        if idx_start == len_cs {
            if target == prev {
                res.push(cur_res.to_owned());
                return;
            }
        }
        for idx in idx_start..len_cs {
            if idx != idx_start && chs[idx_start] == '0' {
                break;
            }
            let cur: i64 = Self::convert(idx_start, idx + 1, &chs);
            if idx_start == 0 {
                Self::backtrack(
                    format!("{}{}", cur_res, cur),
                    cur,
                    cur,
                    idx + 1,
                    chs,
                    target,
                    res,
                );
            } else {
                Self::backtrack(
                    format!("{}{}{}", cur_res, Self::SIGN_PLUS, cur),
                    prev + cur,
                    cur,
                    idx + 1,
                    chs,
                    target,
                    res,
                );

                Self::backtrack(
                    format!("{}{}{}", cur_res, Self::SIGN_MINUS, cur),
                    prev - cur,
                    -cur,
                    idx + 1,
                    chs,
                    target,
                    res,
                );

                Self::backtrack(
                    format!("{}{}{}", cur_res, Self::SIGN_MULTI, cur),
                    prev - multi + multi * cur,
                    multi * cur,
                    idx + 1,
                    chs,
                    target,
                    res,
                );
            }
        }
    }
    fn convert(lo: usize, hi: usize, chs: &Vec<char>) -> i64 {
        let mut ans: i64 = 0;
        for idx in lo..hi {
            ans *= 10;
            let digit = chs[idx] as i64 - '0' as i64;
            ans += digit;
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let num: String = "123".to_owned();
        let target: i32 = 6;
        let expected: Vec<String> = {
            let mut res = vec!["1*2*3".to_owned(), "1+2+3".to_owned()];
            res.sort();
            res
        };
        let actual: Vec<String> = {
            let mut res = Solution::add_operators(num, target);
            res.sort();
            res
        };
        assert_eq!(expected, actual);
    }
}
