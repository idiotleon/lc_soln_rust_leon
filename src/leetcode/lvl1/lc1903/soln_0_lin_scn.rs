/// https://leetcode.com/problems/largest-odd-number-in-string/
///
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1) / O(`len_n`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let len_n = num.len();

        for (idx, ch) in num.chars().rev().enumerate() {
            let digit = ch as u8 - '0' as u8;

            if digit % 2 == 1 {
                return (&num[..len_n - idx]).to_string();
            }
        }

        "".to_string()
    }
}
