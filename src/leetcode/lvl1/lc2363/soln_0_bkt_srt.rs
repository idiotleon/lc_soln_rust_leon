/// @author: Leon
/// https://leetcode.com/problems/merge-similar-items/
/// Time Complexity:    O(`RANGE`)
/// Space Complexity:   O(`RANGE`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const RANGE: usize = 1e3 as usize + 1;
        let mut freqs: Vec<i32> = vec![0; RANGE];
        for item in items1 {
            let idx_value: usize = item[0] as usize;
            let weight: i32 = item[1];
            freqs[idx_value] += weight;
        }
        for item in items2 {
            let idx_value: usize = item[0] as usize;
            let weight: i32 = item[1];
            freqs[idx_value] += weight;
        }
        return freqs
            .into_iter()
            .enumerate()
            .map(|(idx, value)| vec![idx as i32, value])
            .filter(|v| v[1] > 0)
            .collect::<Vec<Vec<i32>>>();
    }
}
