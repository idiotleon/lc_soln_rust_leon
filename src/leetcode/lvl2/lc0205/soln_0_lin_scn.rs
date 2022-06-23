/// @author: Leon
/// https://leetcode.com/problems/isomorphic-strings/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s` + `len_t`) ~ O(`len_s`)
/// Reference:
/// https://github.com/huanminwu/LeetCode/blob/master/LeetCode_Day_5_Binary_Search.docx
/// https://github.com/huanminwu/LeetCode/blob/master/LeetCode_Day_5_Binary_Search.docx
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let len_s = s.len();
        let len_t = t.len();
        if len_s != len_t {
            return false;
        }
        let mut dict_s: Vec<i32> = vec![-1; 256];
        let mut dict_t: Vec<i32> = vec![-1; 256];
        let chs_s: Vec<char> = s.chars().collect();
        let chs_t: Vec<char> = t.chars().collect();
        for (idx, &ch_s) in chs_s.iter().enumerate() {
            let idx_ch_s = ch_s as usize;
            let idx_ch_t = chs_t[idx] as usize;
            if dict_s[idx_ch_s] != dict_t[idx_ch_t] {
                return false;
            }
            dict_s[idx_ch_s] = idx as i32;
            dict_t[idx_ch_t] = idx as i32;
        }
        true
    }
}
