/// @author: Leon
/// https://leetcode.com/problems/combination-sum/
/// Time Complexity:    O(`_len_cds`!)
/// Space Complexity:   O(`_len_cds`)
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
        let len_n: usize = candidates.len();
        if idx_start >= len_n || target < 0 {
            return;
        }
        if 0 == target {
            paths.push(path.to_vec());
            return;
        }
        for idx in idx_start..len_n {
            path.push(candidates[idx]);
            Self::backtrack(idx, target - candidates[idx], candidates, path, paths);
            path.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let candidates = vec![2, 3, 6, 7];
        let target: i32 = 7;
        let actual = Solution::combination_sum(candidates, target);
        let expected = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(expected, actual);
    }
}
