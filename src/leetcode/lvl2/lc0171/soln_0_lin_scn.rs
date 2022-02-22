/// @author: Leon
/// https://leetcode.com/problems/excel-sheet-column-number/
/// Time Complexity:    O(`_len_ct`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let _len_ct: usize = column_title.len();
        let mut ans: i32 = 0;
        const RANGE: i32 = 26;
        let mut hi: i32 = 1;
        for ch in column_title.chars().rev(){
            ans += (ch as i32 - 'A' as i32 + 1) * hi;
            hi *= RANGE;
        }
        ans
    }
}