use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/goat-latin/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        const SPACE: char = ' ';
        let _len_s: usize = sentence.len();
        let words: Vec<&str> = sentence.split(SPACE).collect();
        let vowels: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']
            .into_iter()
            .collect();
        let mut ans: String = String::new();
        let mut cnt_copy: usize = 1;
        for word in words {
            let chs: Vec<char> = word.chars().collect();
            if vowels.contains(&chs[0]) {
                ans.push_str(word);
            } else {
                ans.push_str(&word[1..]);
                ans.push(chs[0]);
            }
            ans.push_str(&"ma");
            ans.push_str(&("a".repeat(cnt_copy)));
            cnt_copy += 1;
            ans.push(SPACE);
        }
        // to remove the surplus trailing space
        ans.pop();
        return ans;
    }
}
