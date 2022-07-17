/// @author: Leon
/// https://leetcode.com/problems/minimum-deletions-to-make-array-divisible/
/// Time Complexity:    O(`_len_ns` + `_len_nds`) ~ O(max(`_len_ns`, `_len_nds`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        let _len_nds: usize = nums_divide.len();
        let gcd = Self::get_gcd_all(&nums_divide);
        let nums: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        // minor optimization
        if gcd < nums[0] {
            return -1;
        }
        for (idx, num) in nums.into_iter().enumerate() {
            if gcd % num == 0 {
                return idx as i32;
            }
        }
        -1
    }
    fn get_gcd_all(nums: &Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let mut gcd = nums[0];
        for idx in 1..len_ns {
            gcd = Self::get_gcd_two(gcd, nums[idx]);
        }
        gcd
    }
    fn get_gcd_two(num1: i32, num2: i32) -> i32 {
        if num2 == 0 {
            return num1;
        }
        Self::get_gcd_two(num2, num1 % num2)
    }
}
