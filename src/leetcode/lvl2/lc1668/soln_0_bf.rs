/// @author: Leon
/// https://leetcode.com/problems/maximum-repeating-substring/
/// Time Complexity:    O(`_len_sq` ^ 2)
/// Space Complexity:   O(`_len_sq`)
/// Reference:
/// https://leetcode.com/problems/maximum-repeating-substring/solutions/2928527/rust-from-c-using-contains/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let _len_sq: usize = sequence.len();
        let mut cnt: i32 = 0;
        let mut seg: String = word.to_owned();
        while sequence.contains(&seg) {
            seg.push_str(&word);
            cnt += 1;
        }
        return cnt;
    }
}
