/// @author: Leon
/// https://leetcode.com/problems/find-the-kth-largest-integer-in-the-array/
/// Time Complexity:    O(`_len_n` * lg(`_len_n`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        let len_n: usize = nums.len();
        let sorted: Vec<String> = {
            let mut sorted = nums;
            sorted.sort_unstable_by(|a, b| {
                if a.len() == b.len() {
                    a.partial_cmp(&b).unwrap()
                } else {
                    a.len().partial_cmp(&b.len()).unwrap()
                }
            });
            sorted
        };
        sorted[len_n - k as usize].to_owned()
    }
}
