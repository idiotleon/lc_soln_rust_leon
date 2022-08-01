/// @author: Leon
/// https://leetcode.com/problems/find-permutation/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_permutation(s: String) -> Vec<i32> {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        const D: char = 'D';
        let mut ans: Vec<i32> = {
            let mut res: Vec<i32> = vec![0; len_s + 1];
            for idx in 0..=len_s {
                res[idx] = idx as i32 + 1;
            }
            res
        };
        let mut lo: usize = 0;
        while lo < len_s {
            let mut hi: usize = lo;
            while hi < len_s && chs[hi] == D {
                hi += 1;
            }
            let _ = &ans[lo..=hi].reverse();
            lo = hi + 1;
        }
        ans
    }
}
