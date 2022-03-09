/// @author: Leon
/// https://leetcode.com/problems/cells-in-a-range-on-an-excel-sheet/
/// Time Complexity:    O((`hi_ch` - `lo_ch`) * (`hi_num` - `lo_num`))
/// Space Complexity:   O(1) / O((`hi_ch` - `lo_ch`) * (`hi_num` - `lo_num`))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let chs: Vec<char> = s.chars().collect();
        let lo_ch: char = chs[0];
        let lo_num: u8 = chs[1] as u8 - '0' as u8;
        let hi_ch: char = chs[3];
        let hi_num: u8 = chs[4] as u8 - '0' as u8;
        let mut ans: Vec<String> = Vec::new();
        for ch in lo_ch..=hi_ch {
            for num in lo_num..=hi_num {
                ans.push(format!("{}{}", ch, num));
            }
        }
        ans
    }
}
