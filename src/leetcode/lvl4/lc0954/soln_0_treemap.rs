use std::collections::BTreeMap;

/// @author: Leon
/// https://leetcode.com/problems/array-of-doubled-pairs/
/// Time Complexity:    O(`_len_n` * lg(`_len_n`))
/// Space Complexity:   O(`_len_n`)
/// Reference:
/// https://leetcode.com/problems/array-of-doubled-pairs/discuss/203183/JavaC%2B%2BPython-Match-from-the-Smallest-or-Biggest-100
/// https://leetcode.com/problems/array-of-doubled-pairs/discuss/886224/Rust-translated-24ms-100
/// https://leetcode.com/problems/array-of-doubled-pairs/discuss/1396885/Rust-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_reorder_doubled(nums: Vec<i32>) -> bool {
        let _len_n = nums.len();
        let mut num_to_freq = BTreeMap::<i32, i32>::new();
        for &num in &nums {
            *num_to_freq.entry(num).or_default() += 1;
        }
        let keys = num_to_freq.keys().copied().collect::<Vec<i32>>();
        for key in keys {
            if num_to_freq[&key] == 0 {
                continue;
            }
            let expected = if key < 0 { key / 2 } else { key * 2 };
            if key < 0 && key & 1 == 1
                || num_to_freq[&key] > *num_to_freq.get(&expected).unwrap_or(&0)
            {
                return false;
            }
            *num_to_freq.entry(expected).or_default() -= num_to_freq[&key];
        }
        true
    }
}
