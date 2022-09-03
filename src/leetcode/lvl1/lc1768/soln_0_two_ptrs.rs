/// @author: Leon
/// https://leetcode.com/problems/merge-strings-alternately/
/// Time Complexity:    O(max(`len1`, `len2`))
/// Space Complexity:   O(max(`len1`, `len2` ))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let len1: usize = word1.len();
        let len2: usize = word2.len();
        let chs1: Vec<char> = word1.chars().collect();
        let chs2: Vec<char> = word2.chars().collect();
        let mut idx1: usize = 0;
        let mut idx2: usize = 0;
        let mut ans: String = "".to_owned();
        while idx1 < len1 || idx2 < len2 {
            if idx1 < len1 {
                ans.push(chs1[idx1]);
                idx1 += 1;
            }
            if idx2 < len2 {
                ans.push(chs2[idx2]);
                idx2 += 1;
            }
        }
        return ans;
    }
}
