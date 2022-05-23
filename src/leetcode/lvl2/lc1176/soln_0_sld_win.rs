/// @author: Leon
/// https://leetcode.com/problems/diet-plan-performance/
/// Time Complexity:    O(`_len_cls`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn diet_plan_performance(calories: Vec<i32>, k: i32, lower: i32, upper: i32) -> i32 {
        let _len_cls: usize = calories.len();
        let ku: usize = k as usize;
        let mut sum: i32 = 0;
        let mut pts: i32 = 0;
        for (idx, &cl) in calories.iter().enumerate() {
            sum += cl;
            if idx + 1 >= ku {
                if sum > upper {
                    pts += 1;
                } else if sum < lower {
                    pts -= 1;
                }
                sum -= calories[idx + 1 - ku];
            }
        }
        pts
    }
}
