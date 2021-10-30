/// https://leetcode.com/problems/maximum-population-year/
///
/// Time Complexity:    O(`RANGE`)
/// Space Complexity:   O(`RANGE`) ~ O(1)
///
/// Reference:
/// https://leetcode.com/problems/maximum-population-year/discuss/1198838/Java-Array-Solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        const RANGE: usize = 2050 - 1950 + 1;
        const YEAR_BASE: usize = 1950;
        let mut timeline: Vec<i32> = vec![0; RANGE];
        for log in logs.iter() {
            let start: usize = log[0] as usize;
            let end: usize = log[1] as usize;
            for i in start..end {
                timeline[i - YEAR_BASE] += 1;
            }
        }
        let mut max: i32 = 0;
        let mut max_year: usize = 0;
        for (idx_year, &population) in timeline.iter().enumerate() {
            if population > max {
                max = population;
                max_year = idx_year + YEAR_BASE;
            }
        }
        max_year as i32
    }
}
