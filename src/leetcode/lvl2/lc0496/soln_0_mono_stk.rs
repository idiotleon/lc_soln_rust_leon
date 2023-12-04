use std::collections::{HashMap, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/next-greater-element-i/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn next_greater_element(sub: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let len_nss: usize = sub.len();
        let len_ns: usize = nums.len();
        let val_to_larger: HashMap<i32, i32> = {
            let mut map: HashMap<i32, i32> = HashMap::with_capacity(len_ns);
            let mut stk: VecDeque<i32> = VecDeque::with_capacity(len_ns);
            for num in nums {
                while let Some(&val_top) = stk.back() {
                    if val_top < num {
                        map.insert(stk.pop_back().unwrap(), num);
                    } else {
                        break;
                    }
                }
                stk.push_back(num);
            }
            map
        };
        let ans: Vec<i32> = {
            let mut res: Vec<i32> = Vec::with_capacity(len_nss);
            for num in sub {
                let idx: i32 = *val_to_larger.get(&num).unwrap_or(&-1);
                res.push(idx);
            }
            res
        };
        return ans;
    }
}
