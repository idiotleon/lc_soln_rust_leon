/// @author: Leon
/// https://leetcode.com/problems/shortest-word-distance-iii/
/// Time Complexity:    O(`len_ws` * avg_len_word)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/shortest-word-distance-iii/discuss/67097/12-16-lines-Java-C%2B%2B
/// https://leetcode.com/problems/shortest-word-distance-iii/discuss/67095/Short-Java-solution-10-lines-O(n)-modified-from-Shortest-Word-Distance-I
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_word_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        let len_ws: usize = words_dict.len();
        let mut idx1: usize = len_ws;
        let mut idx2: usize = len_ws;
        let mut shortest: i32 = len_ws as i32;
        for (idx, word) in words_dict.into_iter().enumerate() {
            if &word == &word1 {
                idx1 = idx;
            }
            if &word == &word2 {
                if &word1 == &word2 {
                    idx1 = idx2;
                }
                idx2 = idx;
            }
            if idx1 != len_ws && idx2 != len_ws {
                shortest = std::cmp::min(shortest, (idx1 as i32 - idx2 as i32).abs());
            }
        }
        return shortest;
    }
}
