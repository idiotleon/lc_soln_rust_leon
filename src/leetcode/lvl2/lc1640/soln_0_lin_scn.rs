use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/check-array-formation-through-concatenation/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/check-array-formation-through-concatenation/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_form_array(nums: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let len_ns: usize = nums.len();
        let len_ps: usize = pieces.len();
        let start_to_vec: HashMap<i32, Vec<i32>> = {
            let mut map: HashMap<i32, Vec<i32>> = HashMap::with_capacity(len_ps);
            for piece in pieces {
                map.insert(piece[0], piece);
            }
            map
        };
        let mut idx: usize = 0;
        while idx < len_ns {
            if !start_to_vec.contains_key(&nums[idx]) {
                return false;
            }
            if let Some(expected) = start_to_vec.get(&nums[idx]) {
                for &exp in expected {
                    if exp != nums[idx] {
                        return false;
                    }
                    idx += 1;
                }
            }
        }
        return true;
    }
}
