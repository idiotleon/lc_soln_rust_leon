/// @author: Leon
/// https://leetcode.com/problems/best-meeting-point/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/best-meeting-point/discuss/74186/14ms-java-solution/249422
/// https://leetcode.com/problems/best-meeting-point/discuss/74186/14ms-java-solution
/// https://leetcode.com/problems/best-meeting-point/discuss/74186/14ms-java-solution/77254
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let (rs, cs): (Vec<i32>, Vec<i32>) = {
            let mut rs: Vec<i32> = Vec::with_capacity(len_rs);
            let mut cs: Vec<i32> = Vec::with_capacity(len_cs);
            for r in 0..len_rs {
                for c in 0..len_cs {
                    if grid[r][c] == 1 {
                        rs.push(r as i32);
                        cs.push(c as i32);
                    }
                }
            }
            (rs, cs)
        };
        Self::get_min_distance(rs) + Self::get_min_distance(cs)
    }
    fn get_min_distance(nums: Vec<i32>) -> i32 {
        let len_cs: usize = nums.len();
        let nums = {
            let mut sorted = nums;
            sorted.sort();
            sorted
        };
        let mut distance: i32 = 0;
        let median = nums[len_cs / 2];
        for num in nums {
            distance += (num - median).abs();
        }
        distance
    }
}
