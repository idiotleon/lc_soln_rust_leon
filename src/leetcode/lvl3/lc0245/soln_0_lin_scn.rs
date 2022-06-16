/// @author: Leon
/// https://leetcode.com/problems/shortest-word-distance-iii/
/// Time Complexity:    O(`_len_ws` * avg_len_word)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/shortest-word-distance-iii/discuss/67097/12-16-lines-Java-C%2B%2B
/// https://leetcode.com/problems/shortest-word-distance-iii/discuss/67095/Short-Java-solution-10-lines-O(n)-modified-from-Shortest-Word-Distance-I
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_word_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        let _len_ws: usize = words.len();
        const RANGE: usize = 3 * 1e4 as usize + 7;
        let mut idx1: usize = RANGE;
        let mut idx2: usize = RANGE;
        let mut shortest: usize = RANGE;
        for (idx, word) in words.iter().enumerate() {
            if word == &word1 {
                idx1 = idx;
            }
            if word == &word2 {
                if &word1 == &word2 {
                    idx1 = idx2;
                }
                idx2 = idx;
            }
            if idx1 != RANGE && idx2 != RANGE {
                shortest = std::cmp::min(
                    shortest,
                    if idx1 > idx2 {
                        idx1 - idx2
                    } else {
                        idx2 - idx1
                    },
                );
            }
        }
        shortest as i32
    }
}
