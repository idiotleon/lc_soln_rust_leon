/// @author: Leon
/// https://leetcode.com/problems/remove-duplicate-letters/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(`_len_s`)
/// Reference:
/// https://leetcode.com/problems/remove-duplicate-letters/discuss/889778/Rust-stk-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let _len_s = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut remaining: Vec<u16> = {
            let mut remaining: Vec<u16> = vec![0; 26];
            for &ch in &chs {
                remaining[ch as usize - 'a' as usize] += 1;
            }
            remaining
        };
        let mut used: Vec<u16> = vec![0; 26];
        let ans: &mut String = &mut "".to_owned();
        for ch in chs {
            if used[ch as usize - 'a' as usize] == 0 {
                while !ans.is_empty()
                    && ans.chars().last().unwrap() >= ch
                    && remaining[ans.chars().last().unwrap() as usize - 'a' as usize] > 0
                {
                    used[ans.chars().last().unwrap() as usize - 'a' as usize] -= 1;
                    ans.pop();
                }
                ans.push(ch);
                used[ch as usize - 'a' as usize] += 1;
            }
            remaining[ch as usize - 'a' as usize] -= 1;
        }
        ans.to_owned()
    }
}
