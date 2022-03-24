/// @author: Leon
/// https://leetcode.com/problems/smallest-string-with-a-given-numeric-value/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut res: Vec<char> = vec!['a'; n as usize];
        let mut idx: usize = n as usize - 1;
        let mut value = k - n;
        while value > 25 {
            res[idx] = 'z';
            idx -= 1;
            value -= 25;
        }
        res[idx] = (value as u8 + 'a' as u8) as char;
        res.into_iter().collect::<String>()
    }
}
