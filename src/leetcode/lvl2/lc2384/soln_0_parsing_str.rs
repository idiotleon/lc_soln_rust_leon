/// @author: Leon
/// https://leetcode.com/problems/largest-palindromic-number/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1) / O(`_len_s`)
/// Reference:
/// https://leetcode.com/problems/largest-palindromic-number/discuss/2456655/Python-HashMap-Solution-with-Explanation/1553083
/// https://leetcode.com/problems/largest-palindromic-number/discuss/2456655/Python-HashMap-Solution-with-Explanation
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let _len_s: usize = num.len();
        const IMPL: i32 = -1;
        let freqs: Vec<u32> = {
            let mut freqs: Vec<u32> = vec![0; 10];
            for ch in num.chars() {
                freqs[ch as usize - '0' as usize] += 1;
            }
            freqs
        };
        let mut ans: String = "".to_owned();
        let mut mid: i32 = IMPL;
        for (digit, freq) in freqs.into_iter().enumerate().rev() {
            if freq == 0 {
                continue;
            }
            if freq % 2 == 1 && mid == IMPL {
                mid = digit as i32;
            }
            if digit == 0 && ans.is_empty() {
                break;
            }
            let rep = freq / 2;
            for _ in 0..rep {
                ans.push_str(&digit.to_string());
            }
        }
        let rev: String = ans.clone().chars().rev().collect();
        if mid != IMPL {
            ans.push_str(&mid.to_string());
        }
        ans.push_str(&rev);
        return if ans.is_empty() { "0".to_owned() } else { ans };
    }
}
