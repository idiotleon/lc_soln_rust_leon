/// @author: Leon
/// https://leetcode.com/problems/remove-invalid-parentheses/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    const PAREN_OPEN: char = '(';
    const PAREN_CLOSED: char = ')';
    const IMPS: char = '#';
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let mut chs: Vec<char> = s.chars().collect();
        Self::backtrack(
            0,
            0,
            &mut chs,
            Self::PAREN_OPEN,
            Self::PAREN_CLOSED,
            &mut ans,
        );
        return ans;
    }
    fn backtrack(
        lo_start: usize,
        hi_start: usize,
        chs: &mut Vec<char>,
        paren_open: char,
        paren_closed: char,
        res: &mut Vec<String>,
    ) {
        let len_cs: usize = chs.len();
        let mut stk: i8 = 0;
        for hi in hi_start..len_cs {
            if chs[hi] == paren_open {
                stk += 1;
            }
            if chs[hi] == paren_closed {
                stk -= 1;
            }
            // match statements equivalent to the above if conditions
            // match chs[hi] {
            //     a if a == paren_open => stk += 1,
            //     b if b == paren_closed => stk -= 1,
            //     _ => {}
            // };
            if stk >= 0 {
                continue;
            }
            for lo in lo_start..=hi {
                if chs[lo] == paren_closed && (lo == 0 || chs[lo - 1] != paren_closed) {
                    let original = chs[lo];
                    chs[lo] = Self::IMPS;
                    Self::backtrack(lo, hi + 1, chs, paren_open, paren_closed, res);
                    chs[lo] = original;
                }
            }
            return;
        }
        let mut reversed = {
            let mut tmp = chs.to_vec();
            tmp.reverse();
            tmp
        };
        if paren_open == Self::PAREN_OPEN {
            Self::backtrack(
                0,
                0,
                &mut reversed,
                Self::PAREN_CLOSED,
                Self::PAREN_OPEN,
                res,
            );
        } else {
            res.push(
                reversed
                    .into_iter()
                    .filter(|&ch| ch != Self::IMPS)
                    .collect::<String>(),
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let s: String = "()())()".to_owned();
        let expected: Vec<String> = {
            let mut res: Vec<String> = vec!["(())()".to_owned(), "()()()".to_owned()];
            res.sort();
            res
        };
        let actual: Vec<String> = {
            let mut res: Vec<String> = Solution::remove_invalid_parentheses(s);
            res.sort();
            res
        };
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let s: String = "(a)())()".to_owned();
        let expected: Vec<String> = {
            let mut res: Vec<String> = vec!["(a())()".to_owned(), "(a)()()".to_owned()];
            res.sort();
            res
        };
        let actual: Vec<String> = {
            let mut res: Vec<String> = Solution::remove_invalid_parentheses(s);
            res.sort();
            res
        };
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let s: String = ")(".to_owned();
        let expected: Vec<String> = {
            let mut res: Vec<String> = vec!["".to_owned()];
            res.sort();
            res
        };
        let actual: Vec<String> = {
            let mut res: Vec<String> = Solution::remove_invalid_parentheses(s);
            res.sort();
            res
        };
        assert_eq!(expected, actual);
    }
}
