/// @author: Leon
/// https://leetcode.com/problems/combination-sum/
/// Time Complexity:    O(`len_cds`!)
/// Space Complexity:   O(`len_cds`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let _len_cds: usize = candidates.len();
        let mut path: Vec<i32> = Vec::new();
        let mut paths: Vec<Vec<i32>> = Vec::new();
        Self::backtrack(0, target, &candidates, &mut path, &mut paths);
        paths
    }
    fn backtrack(
        idx_start: usize,
        target: i32,
        candidates: &Vec<i32>,
        path: &mut Vec<i32>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        let len_cds: usize = candidates.len();
        if target < 0 {
            return;
        }
        if target == 0 {
            paths.push(path.to_vec());
            return;
        }
        for idx in idx_start..len_cds {
            path.push(candidates[idx]);
            Self::backtrack(idx, target - candidates[idx], candidates, path, paths);
            path.pop();
        }
    }
}
