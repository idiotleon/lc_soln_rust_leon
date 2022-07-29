/// @author: Leon
/// https://leetcode.com/problems/find-and-replace-pattern/
/// Time Complexity:    O(`len_ws` * `_len_p`)
/// Space Complexity:   O(max(`len_ws`, `_len_p`))
/// Reference:
/// https://leetcode.com/problems/find-and-replace-pattern/discuss/161266/JAVA-3ms-Clear-Code/751351
/// https://leetcode.com/problems/find-and-replace-pattern/discuss/161266/JAVA-3ms-Clear-Code/220787
/// https://leetcode.com/problems/find-and-replace-pattern/discuss/161266/JAVA-3ms-Clear-Code
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let len_ws: usize = words.len();
        let _len_p: usize = pattern.len();
        let chs_p: Vec<char> = pattern.chars().collect::<Vec<char>>();
        let mut ans: Vec<String> = Vec::with_capacity(len_ws);
        for word in &words {
            let chs_w: Vec<char> = word.chars().collect::<Vec<char>>();
            if Self::matches(&chs_w, &chs_p) {
                ans.push(word.to_owned());
            }
        }
        ans
    }
    fn matches(chs_w: &Vec<char>, chs_p: &Vec<char>) -> bool {
        let len_ws: usize = chs_w.len();
        let len_ps: usize = chs_p.len();
        let mut hash_w: Vec<usize> = vec![0; 26];
        let mut hash_p: Vec<usize> = vec![0; 26];
        for idx in 0..std::cmp::min(len_ws, len_ps) {
            let idx_w: usize = chs_w[idx] as usize - 'a' as usize;
            let idx_p: usize = chs_p[idx] as usize - 'a' as usize;
            if hash_w[idx_w] != hash_p[idx_p] {
                return false;
            }
            hash_w[idx_w] = idx + 1;
            hash_p[idx_p] = idx + 1;
        }
        true
    }
}
