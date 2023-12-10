/// @author: Leon
/// https://leetcode.com/problems/double-modular-exponentiation/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        let len_ns: usize = variables.len();
        let mut ans: Vec<i32> = Vec::with_capacity(len_ns);
        for (idx, nums) in variables.into_iter().enumerate() {
            if Self::get_mod(&nums) == target {
                ans.push(idx as i32);
            }
        }
        return ans;
    }
    fn get_mod(nums: &Vec<i32>) -> i32 {
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];
        let md = nums[3];
        let mut total: i32 = 1;
        for _ in 0..b {
            total *= a;
            total %= 10;
        }
        let cur = total;
        total = 1;
        for _ in 0..c {
            total *= cur;
            total %= md;
        }
        return total;
    }
}
