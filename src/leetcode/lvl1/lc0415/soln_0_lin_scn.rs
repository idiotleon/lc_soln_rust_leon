/// @author: Leon
/// https://leetcode.com/problems/add-strings/
///
/// Time Complexity:    O(`len1` + `len2`) ~ O(maxOf(`len1`, `len2`))
/// Space Complexity:   O(`len1` + `len2`) ~ O(maxOf(`len1`, `len2`))
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let len1 = num1.len();
        let len2 = num2.len();

        let chs1: Vec<char> = num1.chars().collect();
        let chs2: Vec<char> = num2.chars().collect();

        let mut idx1 = len1 as i32 - 1;
        let mut idx2 = len2 as i32 - 1;

        let mut ans: String = String::new();
        let mut carry = 0;

        while idx1 >= 0 || idx2 >= 0 || carry > 0 {
            let n1 = if idx1 >= 0 {
                chs1[idx1 as usize] as u8 - '0' as u8
            } else {
                0
            };
            let n2 = if idx2 >= 0 {
                chs2[idx2 as usize] as u8 - '0' as u8
            } else {
                0
            };

            let sum = n1 + n2 + carry;
            let digit = sum % 10;

            ans.push_str(&digit.to_string());

            carry = sum / 10;

            idx1 -= 1;
            idx2 -= 1;
        }

        ans.chars().rev().collect()
    }
}
