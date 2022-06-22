/// @author: Leon
/// https://leetcode.com/problems/permutations-ii/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len_n: usize = nums.len();
        let sorted = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut used: Vec<bool> = vec![false; len_n];
        let mut path: Vec<i32> = Vec::new();
        let mut paths: Vec<Vec<i32>> = Vec::new();
        Self::backtrack(&mut used, &sorted, &mut path, &mut paths);
        paths
    }
    fn backtrack(
        used: &mut Vec<bool>,
        nums: &Vec<i32>,
        path: &mut Vec<i32>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        let len_n: usize = nums.len();
        if path.len() == len_n {
            paths.push(path.to_vec());
            return;
        }
        for idx in 0..len_n {
            if used[idx] {
                continue;
            }
            if idx > 0 && !used[idx - 1] && nums[idx - 1] == nums[idx] {
                continue;
            }
            path.push(nums[idx]);
            used[idx] = true;
            Self::backtrack(used, nums, path, paths);
            path.pop();
            used[idx] = false;
        }
    }
}
