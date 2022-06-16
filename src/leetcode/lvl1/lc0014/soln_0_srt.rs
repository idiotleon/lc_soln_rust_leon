/// @author: Leon
/// https://leetcode.com/problems/longest-common-prefix/
/// Time Complexity:    O(`len_ss` * lg(`len_ss`))
/// Space Complexity:   O(avg_len_str)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let len_ss: usize = strs.len();
        let sorted: Vec<String> = {
            let mut sorted = strs;
            sorted.sort();
            sorted
        };
        let chs1: Vec<char> = sorted[0].chars().collect();
        let len1: usize = chs1.len();
        let chs2: Vec<char> = sorted[len_ss - 1].chars().collect();
        let len2: usize = chs2.len();
        let ans: &mut String = &mut "".to_owned();
        for idx in 0..len1 {
            if idx < len2 && chs1[idx] == chs2[idx] {
                ans.push(chs1[idx]);
            } else {
                break;
            }
        }
        ans.to_owned()
    }
}
