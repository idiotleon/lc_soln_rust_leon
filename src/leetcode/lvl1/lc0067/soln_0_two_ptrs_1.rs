/// @author: Leon
/// https://leetcode.com/problems/add-binary/
/// Time Complexity:    O(max(`len_a`, `len_b`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/add-binary/discuss/253415/Rust-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let chs_a: Vec<char> = a.chars().collect();
        let len_a = chs_a.len();
        let chs_b: Vec<char> = b.chars().collect();
        let len_b = chs_b.len();
        let mut carry = 0;
        let mut idx_a: usize = len_a;
        let mut idx_b: usize = len_b;
        let mut ans: Vec<char> = vec![];
        while idx_a > 0 || idx_b > 0 || carry > 0 {
            let mut sum: u8 = 0;
            if idx_a > 0 {
                idx_a -= 1;
                sum += chs_a[idx_a] as u8 - '0' as u8;
            }
            if idx_b > 0 {
                idx_b -= 1;
                sum += chs_b[idx_b] as u8 - '0' as u8;
            }
            sum += carry;
            let digit = sum % 2;
            ans.push((digit + '0' as u8) as char);
            carry = sum / 2;
        }
        return ans.iter().rev().collect::<String>();
    }
}
