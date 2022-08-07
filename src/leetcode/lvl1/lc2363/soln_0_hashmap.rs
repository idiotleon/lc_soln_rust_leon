use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/merge-similar-items/
/// Time Complexity:    O(`_len1` + `_len2`)
/// Space Complexity:   O(`_len1` + `_len2`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let _len1: usize = items1.len();
        let _len2: usize = items2.len();
        let mut value_to_weight: HashMap<i32, i32> = HashMap::new();
        for item in items1 {
            let value = item[0];
            let weight = item[1];
            *value_to_weight.entry(value).or_default() += weight;
        }
        for item in items2 {
            let value = item[0];
            let weight = item[1];
            *value_to_weight.entry(value).or_default() += weight;
        }
        let mut ans = value_to_weight
            .into_iter()
            .map(|(value, weight)| vec![value, weight])
            .collect::<Vec<Vec<i32>>>();
        ans.sort_by_key(|e| e[0]);
        return ans;
    }
}
