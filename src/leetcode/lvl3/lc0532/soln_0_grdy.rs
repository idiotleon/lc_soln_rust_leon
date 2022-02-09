use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/k-diff-pairs-in-an-array/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`_len_n`)
/// Referennce:
/// https://leetcode.com/problems/k-diff-pairs-in-an-array/discuss/100098/Java-O(n)-solution-one-Hashmap-easy-to-understand/146186
/// https://leetcode.com/problems/k-diff-pairs-in-an-array/discuss/100098/Java-O(n)-solution-one-Hashmap-easy-to-understand
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let _len_n: usize = nums.len();
        let mut num_to_freq: HashMap<i32, i32> = HashMap::new();
        let mut cnt: i32 = 0;
        for num in nums {
            match num_to_freq.get_mut(&num) {
                Some(freq) => {
                    if k == 0 && *freq == 1 {
                        cnt += 1;
                    }
                    *freq += 1;
                }
                None => {
                    if num_to_freq.contains_key(&(num - k)) {
                        cnt += 1;
                    }
                    if num_to_freq.contains_key(&(num + k)) {
                        cnt += 1;
                    }
                    num_to_freq.insert(num, 1);
                }
            }
        }
        cnt
    }
}
