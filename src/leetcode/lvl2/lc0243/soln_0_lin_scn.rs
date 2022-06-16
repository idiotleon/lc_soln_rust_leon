/// @author: Leon
/// https://leetcode.com/problems/shortest-word-distance/
/// Time Complexity:    O(`_len_ws` * avg_len_word)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/shortest-word-distance/discuss/66931/AC-Java-clean-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        let _len_ws: usize = words.len();
        const RANGE: usize = 3 * 1e4 as usize + 7;
        let mut idx1: usize = RANGE;
        let mut idx2: usize = RANGE;
        let mut shortest: usize = RANGE;
        for (idx, word) in words.iter().enumerate(){
            if word == &word1{
                idx1 = idx;
            }
            if word == &word2{
                idx2 = idx;
            }
            if idx1 != RANGE && idx2 != RANGE{
                shortest = std::cmp::min(shortest, 
                    if idx1 > idx2 { idx1 - idx2 } else {idx2 - idx1});
            }
        }
        shortest as i32
    }
}