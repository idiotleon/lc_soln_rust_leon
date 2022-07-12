/// @author: Leon
/// https://leetcode.com/problems/longest-palindrome/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let _len_s: usize = s.len();
        let freqs: Vec<u16> = {
            let mut freqs: Vec<u16> = vec![0; 256];
            for &b in s.as_bytes() {
                freqs[b as usize] += 1;
            }
            freqs
        };
        let mut ans: u16 = 0;
        let mut added: bool = false;
        for freq in freqs {
            if freq % 2 == 0 {
                ans += freq;
            } else {
                ans += freq - 1;
                if !added {
                    ans += 1;
                    added = true;
                }
            }
        }
        ans as i32
    }
}
