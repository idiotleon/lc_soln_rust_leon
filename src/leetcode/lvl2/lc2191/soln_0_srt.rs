/// @author: Leon
/// https://leetcode.com/problems/sort-the-jumbled-numbers/
/// Time Complexity:    O(`_len_ns` * lg(`_len_ns`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_jumbled(mappings: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let _len_ns: usize = nums.len();
        let nums: Vec<i32> = {
            let mut nums: Vec<i32> = nums;
            nums.sort_by_key(|&n| Self::jumble(n, &mappings));
            nums
        };
        return nums;
    }
    fn jumble(num: i32, mappings: &Vec<i32>) -> i32 {
        if num == 0 {
            return mappings[0];
        }
        let mut num = num;
        let mut res: i32 = 0;
        let mut cnt: i32 = 0;
        while num > 0 {
            let digit: i32 = mappings[(num % 10) as usize];
            let tens = {
                let mut tens: i32 = 1;
                for _ in 0..cnt {
                    tens *= 10;
                }
                tens
            };
            res += tens * digit;
            cnt += 1;
            num /= 10;
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let mappings: Vec<i32> = vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6];
        let nums: Vec<i32> = vec![991, 338, 38];
        let expected: Vec<i32> = vec![338, 38, 991];
        let actual: Vec<i32> = Solution::sort_jumbled(mappings, nums);
        assert_eq!(expected, actual);
    }
}
