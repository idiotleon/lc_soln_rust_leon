/// @author: Leon
/// https://leetcode.com/problems/partition-labels/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/partition-labels/discuss/113259/Java-2-pass-O(n)-time-O(1)-space-extending-end-pointer-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let _len_s = s.len();
        let chs: Vec<char> = s.chars().collect();
        let last_indices: Vec<usize> = {
            let mut last_indices: Vec<usize> = vec![0; 26];
            for (idx, &ch) in chs.iter().enumerate() {
                last_indices[ch as usize - 'a' as usize] = idx;
            }
            last_indices
        };
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        let mut ans: Vec<i32> = Vec::new();
        for (idx, ch) in chs.into_iter().enumerate() {
            hi = std::cmp::max(hi, last_indices[ch as usize - 'a' as usize]);
            if hi == idx {
                ans.push((hi - lo + 1) as i32);
                lo = hi + 1;
            }
        }
        ans
    }
}
