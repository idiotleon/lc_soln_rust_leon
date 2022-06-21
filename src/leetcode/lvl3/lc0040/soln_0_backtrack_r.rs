/// @author: Leon
/// https://leetcode.com/problems/combination-sum-ii/
/// Time Complexity:    O(`len_cds`!)
/// Space Complexity:   O(`len_cds`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len_cds: usize = candidates.len();
        let sorted = {
            let mut candidates = candidates;
            candidates.sort();
            candidates
        };
        let mut path: Vec<i32> = Vec::new();
        let mut paths: Vec<Vec<i32>> = Vec::new();
        let mut visited: Vec<bool> = vec![false; len_cds];
        Self::backtrack(0, target, &sorted, &mut visited, &mut path, &mut paths);
        paths
    }
    fn backtrack(
        idx_start: usize,
        target: i32,
        candidates: &Vec<i32>,
        visited: &mut Vec<bool>,
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
            if visited[idx] || (idx > idx_start && candidates[idx - 1] == candidates[idx]) {
                continue;
            }
            path.push(candidates[idx]);
            visited[idx] = true;
            Self::backtrack(
                idx + 1,
                target - candidates[idx],
                candidates,
                visited,
                path,
                paths,
            );
            path.pop();
            visited[idx] = false;
        }
    }
}
