use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(5 ^ 2) ~ O(1)
/// Reference:
/// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/discuss/532101/Java-o(n)-one-pass-solution.-Easy-to-understand./564392
/// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/discuss/532101/Java-o(n)-one-pass-solution.-Easy-to-understand.
/// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/discuss/531840/JavaC%2B%2BPython-One-Pass
/// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/discuss/534135/C%2B%2BJava-with-picture
/// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/discuss/534210/Dew-It-or-Simple-illustration-for-THE-trick
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let _len_s: usize = s.len();
        let vowel_to_bit_idx: HashMap<char, u32> = {
            let mut map: HashMap<char, u32> = HashMap::with_capacity(5);
            for (idx, ch) in "aeiou".chars().enumerate() {
                let bitmask = 1 << (idx as u32);
                map.insert(ch, bitmask);
            }
            map
        };
        // or equivalently
        // let vowel_to_bit_idx: HashMap<char, u32> = {
        //     let mut map: HashMap<char, u32> = HashMap::with_capacity(5);
        //     map.insert('a', 1);
        //     map.insert('e', 2);
        //     map.insert('i', 4);
        //     map.insert('o', 8);
        //     map.insert('u', 16);
        //     map
        // };
        let mut state_to_first_idx: HashMap<u32, i32> = {
            let mut map = HashMap::new();
            map.insert(0, -1);
            map
        };
        let mut state: u32 = 0;
        let mut longest: i32 = 0;
        for (idx, ch) in s.chars().enumerate() {
            if let Some(idx_to_flip) = vowel_to_bit_idx.get(&ch) {
                state ^= idx_to_flip;
            }
            let first_idx = state_to_first_idx.entry(state).or_insert(idx as i32);
            longest = std::cmp::max(longest, idx as i32 - *first_idx);
        }
        longest
    }
}
