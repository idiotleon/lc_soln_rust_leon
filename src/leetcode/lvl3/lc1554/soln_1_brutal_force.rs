/// @author: Leon
/// https://leetcode.com/problems/strings-differ-by-one-character/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// This approach leads to TLE
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn differ_by_one(dict: Vec<String>) -> bool {
        let len_d: usize = dict.len();
        for lo in 0..len_d - 1 {
            for hi in lo + 1..len_d {
                if Self::distinct_by_one(&dict[lo], &dict[hi]) {
                    return true;
                }
            }
        }
        false
    }
    fn distinct_by_one(word1: &str, word2: &str) -> bool {
        let mut cnt = 0;
        let chs2: Vec<char> = word2.chars().collect();
        for (idx, ch1) in word1.chars().into_iter().enumerate() {
            if ch1 != chs2[idx] {
                cnt += 1;
            }
        }
        cnt == 1
    }
}
