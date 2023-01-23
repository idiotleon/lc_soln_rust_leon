use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/minimum-index-sum-of-two-lists/
/// Time Complexity:    O(`len1` + `len2`) ~ O(max(`len1`, `len2`))
/// Space Complexity:   O(`len1` + `len2`) ~ O(max(`len1`, `len2`))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let len1: usize = list1.len();
        let len2: usize = list2.len();
        if len1 < len2 {
            return Self::find_restaurant(list2, list1);
        }
        let s_to_idx: HashMap<String, usize> = {
            let mut map: HashMap<String, usize> = HashMap::with_capacity(len2);
            for (idx, s) in list2.into_iter().enumerate() {
                map.insert(s, idx);
            }
            map
        };
        let mut min_cur: usize = len1 + len2;
        let mut ans: Vec<String> = Vec::with_capacity(len2);
        for (idx1, s) in list1.into_iter().enumerate() {
            if let Some(idx2) = s_to_idx.get(&s) {
                let sum = idx1 + idx2;
                if sum == min_cur {
                    ans.push(s);
                } else if sum < min_cur {
                    ans.clear();
                    ans.push(s);
                    min_cur = sum;
                }
            }
        }
        return ans;
    }
}
