/// @author: Leon
/// https://leetcode.com/problems/maximum-population-year/
/// Time Complexity:    O(`RANGE`)
/// Space Complexity:   O(`RANGE`) ~ O(1)
/// Reference:
/// https://leetcode.com/problems/maximum-population-year/discuss/1198838/Java-Array-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        const RANGE: usize = 2050 - 1950 + 1;
        const YEAR_BASE: usize = 1950;
        let timeline: Vec<i32> = {
            let mut timeline = vec![0; RANGE];
            for log in logs {
                let start: usize = log[0] as usize;
                let end: usize = log[1] as usize;
                for i in start..end {
                    timeline[i - YEAR_BASE] += 1;
                }
            }
            timeline
        };
        let mut max: i32 = 0;
        let mut max_year: usize = 0;
        for (idx_year, population) in timeline.into_iter().enumerate() {
            if population > max {
                max = population;
                max_year = idx_year + YEAR_BASE;
            }
        }
        max_year as i32
    }
}
