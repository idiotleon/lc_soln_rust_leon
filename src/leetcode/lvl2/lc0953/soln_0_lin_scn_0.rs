/// https://leetcode.com/problems/verifying-an-alien-dictionary/
/// 
/// Time Complexity:    O()
/// Space Complexity:   O()
/// 
/// Reference:
/// https://leetcode.com/problems/verifying-an-alien-dictionary/discuss/987952/Idiomatic-Rust
use std::cmp::Ordering;
use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let dict: HashMap<char, usize> = order
            .chars()
            .enumerate()
            .map(|(idx, ch)| (ch, idx))
            .collect();

        'toploop: for word_pair in words.windows(2) {
            for (ch1, ch2) in word_pair[0].chars().zip(word_pair[1].chars()) {
                match dict[&ch1].cmp(&dict[&ch2]) {
                    Ordering::Equal => {}
                    Ordering::Greater => return false,
                    Ordering::Less => continue 'toploop,
                }
            }

            if word_pair[0].len() > word_pair[1].len() {
                return false;
            }
        }
        true
    }
}
