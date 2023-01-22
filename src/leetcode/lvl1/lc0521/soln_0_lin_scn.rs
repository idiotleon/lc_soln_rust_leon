/// @author: Leon
/// https://leetcode.com/problems/longest-uncommon-subsequence-i/
/// Time Cmoplexity:    O(`len_a` + `len_b`) ~ O(max(`len_a`, `len_b`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        let len_a: usize = a.len();
        let len_b: usize = b.len();
        return if &a == &b {
            -1
        } else {
            std::cmp::max(len_a, len_b) as i32
        };
    }
}
