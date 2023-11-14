use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(`_len_s`)
/// Reference:
/// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/editorial/?envType=daily-question&envId=2023-11-14
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        const RANGE: usize = 1e5 as usize;
        const IMPS: usize = RANGE + 7;
        let _len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let (first_indices, last_indices): (Vec<usize>, Vec<usize>) = {
            let mut first: Vec<usize> = vec![IMPS; 26];
            let mut last: Vec<usize> = vec![IMPS; 26];
            for (idx, &ch) in chs.iter().enumerate() {
                let idx_ch: usize = ch as usize - 'a' as usize;
                if first[idx_ch] == IMPS {
                    first[idx_ch] = idx;
                }
                last[idx_ch] = idx;
            }
            (first, last)
        };
        let mut cnt: i32 = 0;
        for lo in 0..26 {
            if first_indices[lo] == IMPS {
                continue;
            }
            let mut between: HashSet<char> = HashSet::with_capacity(26);
            for hi in first_indices[lo] + 1..last_indices[lo] {
                between.insert(chs[hi]);
            }
            cnt += between.len() as i32;
        }
        return cnt;
    }
}
