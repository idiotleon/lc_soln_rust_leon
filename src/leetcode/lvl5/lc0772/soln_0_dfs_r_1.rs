use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/basic-calculator-iii/
/// Time Complexity:    O(`len_s`)
/// Space Compleixty:   O(`len_s`)
/// Reference:
/// https://leetcode.com/problems/basic-calculator-iii/discuss/152092/O(n)-Java-Recursive-Simple-Solution/227715
struct Solution;

#[allow(dead_code)]
impl Solution {
    const PAREN_OPEN: char = '(';
    const PAREN_CLOSED: char = ')';
    const SPACE: char = ' ';
    const SIGN_PLUS: char = '+';
    const SIGN_MINUS: char = '-';
    const SIGN_MULTIPLY: char = '*';
    const SIGN_DIVIDE: char = '/';
    pub fn calculate(s: String) -> i32 {
        let len_s = s.len();
        let mut tokens: VecDeque<char> = VecDeque::with_capacity(len_s);
        for ch in s.chars() {
            if ch != Self::SPACE {
                tokens.push_back(ch);
            }
        }
        tokens.push_back(Self::SIGN_PLUS);
        Self::dfs(&mut tokens)
    }
    fn dfs(tokens: &mut VecDeque<char>) -> i32 {
        let mut op: char = Self::SIGN_PLUS;
        let mut num: i32 = 0;
        let mut sum: i32 = 0;
        let mut prev: i32 = 0;
        while let Some(ch) = tokens.pop_front() {
            if '0' <= ch && ch <= '9' {
                num = num * 10 + (ch as i32 - '0' as i32);
                continue;
            }
            if ch == Self::PAREN_OPEN {
                num = Self::dfs(tokens);
                continue;
            }
            if op == Self::SIGN_PLUS {
                sum += prev;
                prev = num;
            }
            if op == Self::SIGN_MINUS {
                sum += prev;
                prev = -num;
            }
            if op == Self::SIGN_MULTIPLY {
                prev *= num;
            }
            if op == Self::SIGN_DIVIDE {
                prev /= num;
            }
            if ch == Self::PAREN_CLOSED {
                break;
            }
            op = ch;
            num = 0;
        }
        sum + prev
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1_should_return_expected() {
        let s: String = "1+1".to_owned();
        let actual: i32 = Solution::calculate(s);
        let expected: i32 = 2;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_sample_input_2_should_return_expected() {
        let s: String = "6-4/2".to_owned();
        let actual: i32 = Solution::calculate(s);
        let expected: i32 = 4;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_sample_input_3_should_return_expected() {
        let s: String = "2*(5+5*2)/3+(6/2+8)".to_owned();
        let actual: i32 = Solution::calculate(s);
        let expected: i32 = 21;
        assert_eq!(expected, actual);
    }
}
