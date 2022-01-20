/// @author: Leon
/// https://leetcode.com/problems/plus-one/
/// Time Complexity:    O(`len_ds`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let len_ds: usize = digits.len();
        let ans: Vec<i32> = {
            let mut res: Vec<i32> = Vec::with_capacity(len_ds + 1);
            let mut carry: i32 = 0;
            for (idx, digit) in digits.into_iter().enumerate().rev() {
                let sum = if idx == len_ds - 1 {
                    1 + carry + digit
                } else {
                    carry + digit
                };
                res.push(sum % 10);
                carry = sum / 10;
            }
            if carry > 0 {
                res.push(carry);
            }
            res.reverse();
            res
        };
        ans
    }
}
