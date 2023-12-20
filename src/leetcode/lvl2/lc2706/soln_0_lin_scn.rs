/// @author: Leon
/// https://leetcode.com/problems/buy-two-chocolates/
/// Time Complexity:    O(`_len_ps`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let _len_ps: usize = prices.len();
        const RANGE: i32 = 100 + 7;
        let (min, min_sec): (i32, i32) = {
            let mut min: i32 = RANGE;
            let mut min_sec: i32 = RANGE;
            for price in prices {
                if price < min {
                    min_sec = min;
                    min = price;
                } else if price < min_sec {
                    min_sec = price;
                }
            }
            (min, min_sec)
        };
        let sum: i32 = (min + min_sec) as i32;
        return if sum <= money { money - sum } else { money };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let prices: Vec<i32> = vec![1, 2, 2];
        let money: i32 = 3;
        let expected: i32 = 0;
        let actual: i32 = Solution::buy_choco(prices, money);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let prices: Vec<i32> = vec![3, 2, 3];
        let money: i32 = 3;
        let expected: i32 = 3;
        let actual: i32 = Solution::buy_choco(prices, money);
        assert_eq!(expected, actual);
    }
}
