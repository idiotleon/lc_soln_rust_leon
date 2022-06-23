use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/strings-differ-by-one-character/
/// Time Complexity:    O(`len_ws` * (`len_w` ^ 2))
/// Space Complexity:   O(`len_w` ^ 2)
/// Reference:
/// https://leetcode.com/problems/strings-differ-by-one-character/discuss/805033/Easy-Java-Hashset-O(Nm2)-solution-beats-83/913959
/// https://leetcode.com/problems/strings-differ-by-one-character/discuss/805033/Easy-Java-Hashset-O(Nm2)-solution-beats-83
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn differ_by_one(words: Vec<String>) -> bool {
        let len_ws: usize = words.len();
        if len_ws < 2 {
            return false;
        }
        let len_w: usize = words[0].len();
        for idx in 0..len_w {
            let mut seen: HashSet<String> = HashSet::new();
            for word in words.iter() {
                let substr = word[..idx].to_owned() + &word[idx + 1..];
                if !seen.insert(substr) {
                    return true;
                }
            }
        }
        false
    }
}
