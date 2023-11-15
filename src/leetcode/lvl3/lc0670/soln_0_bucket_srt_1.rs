/// @author: Leon
/// https://leetcode.com/problems/maximum-swap/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/maximum-swap/discuss/107068/Java-simple-solution-O(n)-time
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut digits: Vec<char> = num.to_string().chars().collect();
        let buckets: Vec<usize> = {
            let mut res: Vec<usize> = vec![0; 10];
            for (idx, &ch) in digits.iter().enumerate() {
                let digit = ch as usize - '0' as usize;
                res[digit] = idx;
            }
            res
        };
        let mut largest_digit: usize = 9;
        for (idx, &ch) in digits.iter().enumerate() {
            let digit = ch as usize - '0' as usize;
            for k in (1 + digit..=largest_digit).rev() {
                if buckets[k] > idx {
                    digits.swap(idx, buckets[k]);
                    return digits
                        .into_iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();
                }
            }
            largest_digit = digit;
        }
        return num;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let num: i32 = 2736;
        let expected: i32 = 7236;
        let actual: i32 = Solution::maximum_swap(num);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let num: i32 = 9973;
        let expected: i32 = 9973;
        let actual: i32 = Solution::maximum_swap(num);
        assert_eq!(expected, actual);
    }
}
