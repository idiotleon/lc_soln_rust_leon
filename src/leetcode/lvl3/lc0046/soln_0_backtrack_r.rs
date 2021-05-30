/// @author: Leon
/// https://leetcode.com/problems/permutations/
///
/// Time Complexity:    O()
/// Space Complexity:   O()
///
/// Reference:
/// https://leetcode.com/problems/subsets/discuss/27281/A-general-approach-to-backtracking-questions-in-Java-(Subsets-Permutations-Combination-Sum-Palindrome-Partitioning)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len_n: usize = nums.len();
        let mut used: Vec<bool> = vec![false; len_n];
        let mut path: Vec<i32> = Vec::new();
        let mut paths: Vec<Vec<i32>> = Vec::new();
        Self::backtrack(&mut used, &nums, &mut path, &mut paths);
        paths
    }
    fn backtrack(
        mut used: &mut Vec<bool>,
        nums: &Vec<i32>,
        mut path: &mut Vec<i32>,
        mut paths: &mut Vec<Vec<i32>>,
    ) {
        if path.len() == nums.len() {
            paths.push(path.clone());
            return;
        }

        for (idx, &num) in nums.iter().enumerate() {
            if used[idx] {
                continue;
            }
            path.push(num);
            used[idx] = true;
            Self::backtrack(&mut used, &nums, &mut path, &mut paths);
            path.pop();
            used[idx] = false;
        }
    }
}
