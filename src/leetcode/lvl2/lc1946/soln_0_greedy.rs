/// @author: Leon
/// https://leetcode.com/problems/largest-number-after-mutating-substring/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
/// Reference:
/// https://leetcode.com/problems/largest-number-after-mutating-substring/discuss/1360753/Mutation-must-be-continuous
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_number(num_str: String, changes: Vec<i32>) -> String {
        let len_n = num_str.len();
        let mut mutated = false;
        let mut chs: Vec<char> = num_str.chars().collect();
        for idx in 0..len_n {
            let digit = chs[idx] as u8 - '0' as u8;
            chs[idx] = ('0' as u8 + std::cmp::max(digit, changes[digit as usize] as u8)) as char;
            if (changes[digit as usize] as u8) < digit && mutated {
                break;
            }
            mutated |= digit < changes[digit as usize] as u8;
        }
        chs.iter().collect::<String>()
    }
}
