/// @author: Leon
/// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/
/// Time Complexity:    O(`_len_ns` * lg(`_len_ns`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let _len_ns: usize = arr.len();
        let sorted: Vec<i32> = {
            let mut nums = arr;
            nums.sort_by(|a, b| {
                let cnt_a = a.count_ones();
                let cnt_b = b.count_ones();
                if cnt_a == cnt_b {
                    a.cmp(&b)
                } else {
                    cnt_a.cmp(&cnt_b)
                }
            });
            nums
        };
        return sorted;
    }
}
