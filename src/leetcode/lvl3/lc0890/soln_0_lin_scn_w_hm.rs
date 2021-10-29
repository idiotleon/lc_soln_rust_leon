/// https://leetcode.com/problems/find-and-replace-pattern/
///
/// Time Complexity:    O(`_n_words` * L)
/// Space Complexity:   O(`_n_words` * L)
///
/// Reference:
/// https://leetcode.com/problems/find-and-replace-pattern/discuss/543439/Rust-using-two-maps
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let _n_words = words.len();
        let mut ans = vec![];
        let ptrn_chs: Vec<char> = pattern.chars().collect();
        let match_ptrn = |a: &Vec<char>, b: &Vec<char>| -> bool {
            let mut ch_to_freq = std::collections::HashMap::new();
            (0..a.len()).all(|i| *ch_to_freq.entry(a[i]).or_insert(b[i]) == b[i])
        };
        for word in words.into_iter() {
            let chs: Vec<char> = word.chars().collect();
            let is_match = match_ptrn(&chs, &ptrn_chs) && match_ptrn(&ptrn_chs, &chs);
            if is_match {
                ans.push(word.to_string());
            }
        }
        ans
    }
}
