/// @author: Leon
/// https://leetcode.com/problems/maximum-number-of-coins-you-can-get/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let len_ps: usize = piles.len();
        let sorted: Vec<i32> = {
            let mut res = piles;
            res.sort();
            res
        };
        let mut sum: i32 = 0;
        let mut times: i32 = (len_ps as i32) / 3;
        let mut idx: usize = len_ps - 2;
        while times > 0 {
            sum += sorted[idx];
            idx -= 2;
            times -= 1;
        }
        return sum;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let piles: Vec<i32> = vec![2, 4, 1, 2, 7, 8];
        let expected: i32 = 9;
        let actual: i32 = Solution::max_coins(piles);
        assert_eq!(expected, actual);
    }
}
