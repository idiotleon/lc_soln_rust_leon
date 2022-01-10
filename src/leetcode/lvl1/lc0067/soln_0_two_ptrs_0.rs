/// @author: Leon
/// https://leetcode.com/problems/add-binary/
/// Time Complexity:    O(max(`len_a`, `len_b`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        const MOD: u8 = 2;
        let len_a: usize = a.len();
        let len_b: usize = b.len();
        let chs_a: Vec<char> = a.chars().collect();
        let chs_b: Vec<char> = b.chars().collect();
        let mut carry: u8 = 0;
        let mut idx_a: isize = len_a as isize - 1;
        let mut idx_b: isize = len_b as isize - 1;
        let mut ans: String = "".to_owned();
        while idx_a >= 0 || idx_b >= 0 || carry > 0 {
            let digit_a: u8 = if idx_a >= 0 {
                chs_a[idx_a as usize] as u8 - '0' as u8
            } else {
                0
            };
            let digit_b: u8 = if idx_b >= 0 {
                chs_b[idx_b as usize] as u8 - '0' as u8
            } else {
                0
            };
            let sum: u8 = digit_a + digit_b + carry;
            ans.push((sum % MOD + '0' as u8) as char);
            carry = sum / MOD;
            idx_a -= 1;
            idx_b -= 1;
        }
        ans.chars().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let a: String = "11".to_owned();
        let b: String = "1".to_owned();
        let actual = Solution::add_binary(a, b);
        let expected = "100".to_owned();
        assert_eq!(expected, actual);
    }
}
