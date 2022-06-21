/// @author: Leon
/// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
/// Time Complexity:    O(3 ^ `_len_ds`)
/// Space Complexity:   O(`_len_ds`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIAL_PAD: &'static [&'static str] = &[
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let _len_ds: usize = digits.len();
        let mut ans: Vec<String> = Vec::new();
        Self::backtrack(0, &mut "".to_owned(), digits.as_bytes(), &mut ans);
        ans
    }
    fn backtrack(idx: usize, cur: &mut String, digits: &[u8], res: &mut Vec<String>) {
        let len_ds: usize = digits.len();
        let len_c: usize = cur.len();
        if len_ds == len_c {
            res.push(cur.to_owned());
            return;
        }
        for &b in digits {
            cur.push((b - b'0') as char);
            Self::backtrack(1 + idx, cur, digits, res);
            cur.pop();
        }
    }
}
