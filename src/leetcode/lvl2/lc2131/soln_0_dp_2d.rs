/// @author: Leon
/// https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/
/// Time Complexity:    O(`_len_ws`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/discuss/1675343/Python3-Java-C%2B%2B-Counting-Mirror-Words-O(n)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let _len_ws: usize = words.len();
        let mut freqs: Vec<Vec<u16>> = vec![vec![0; 26]; 26];
        let mut ans: i32 = 0;
        for word in words {
            let chs: Vec<char> = word.chars().collect();
            let idx0: usize = chs[0] as usize - 'a' as usize;
            let idx1: usize = chs[1] as usize - 'a' as usize;
            if freqs[idx1][idx0] > 0 {
                ans += 4;
                freqs[idx1][idx0] -= 1;
            } else {
                freqs[idx0][idx1] += 1;
            }
        }
        for idx in 0..26 {
            if freqs[idx][idx] > 0 {
                ans += 2;
                break;
            }
        }
        ans
    }
}
