/// @author: Leon
/// https://leetcode.com/problems/removing-minimum-number-of-magic-beans/
/// Time Complexity:    O(`len_bs`)
/// Space Complexity:   O(`len_bs`)
/// Reference:
/// https://leetcode.com/problems/removing-minimum-number-of-magic-beans/discuss/1766868/Simple-Java-Solution-Using-Sort
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let len_bs: usize = beans.len();
        const RANGE: i64 = 10e10 as i64 + 7;
        let sum: i64 = beans.iter().map(|&e| e as i64).sum::<i64>();
        let beans: Vec<i32> = {
            let mut beans = beans;
            beans.sort();
            beans
        };
        let mut min: i64 = RANGE;
        let mut len: i64 = len_bs as i64;
        for bean in beans {
            min = std::cmp::min(min, sum - len * bean as i64);
            len -= 1;
        }
        min
    }
}
