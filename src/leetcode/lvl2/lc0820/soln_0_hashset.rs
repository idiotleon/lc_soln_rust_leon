use std::collections::HashSet;
/// @author: Leon
/// https://leetcode.com/problems/short-encoding-of-words/
/// Time Complexity:    O(`_len_ws` * avg_len_word)
/// Space Complexity:   O(`_len_ws` * avg_len_word)
/// Reference:
/// https://leetcode.com/problems/short-encoding-of-words/discuss/125811/C%2B%2BJavaPython-Easy-Understood-Solution-with-Explanation
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let _len_ws: usize = words.len();
        let cloned = words.clone();
        let mut set: HashSet<&str> = cloned.iter().map(String::as_ref).into_iter().collect();
        for word in words{
            for idx in 1..word.len(){
                set.remove(&word[idx..]);
            }
        }
        let mut cnt: i32 = 0;
        for word in set{
            cnt += word.len() as i32 + 1;
        }
        cnt
    }
}