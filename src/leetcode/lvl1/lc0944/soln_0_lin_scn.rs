/// @author: Leon
/// https://leetcode.com/problems/delete-columns-to-make-sorted/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * `len_cs`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let len_rs: usize = strs.len();
        let len_cs: usize = strs[0].len();
        let chses: Vec<Vec<char>> = strs.into_iter().map(|str| str.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let mut cnt: i32 = 0;
        for c in 0..len_cs{
            for r in 1..len_rs{
                if chses[r - 1][c] > chses[r][c]{
                    cnt += 1;
                    break;
                }
            }
        }
        return cnt;
    }
}