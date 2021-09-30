/// https://leetcode.com/problems/partition-labels/
///
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
///
/// Reference:
/// https://leetcode.com/problems/partition-labels/discuss/113259/Java-2-pass-O(n)-time-O(1)-space-extending-end-pointer-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        // not used
        // let len_s = s.len();

        let chs: Vec<char> = s.chars().collect();

        let mut last_idxes: Vec<usize> = vec![0; 26];
        for (idx, &ch) in chs.iter().enumerate() {
            last_idxes[ch as usize - 'a' as usize] = idx;
        }

        let mut lo: usize = 0;
        let mut hi: usize = 0;

        let mut ans: Vec<i32> = Vec::new();
        for (idx, &ch) in chs.iter().enumerate() {
            hi = std::cmp::max(hi, last_idxes[ch as usize - 'a' as usize]);

            if hi == idx {
                ans.push((hi - lo + 1) as i32);
                lo = hi + 1;
            }
        }

        ans
    }
}
