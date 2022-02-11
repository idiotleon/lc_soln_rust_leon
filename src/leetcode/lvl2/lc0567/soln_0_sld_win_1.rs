/// @author: Leon
/// https://leetcode.com/problems/permutation-in-string/
/// Time Complexity:    O(`len_2`)
/// Space Complexity:   O(`len_2`)
/// Reference:
/// https://leetcode.com/problems/permutation-in-string/discuss/102588/Java-Solution-Sliding-Window/105954
/// https://leetcode.com/problems/permutation-in-string/discuss/102588/Java-Solution-Sliding-Window
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let len1: usize = s1.len();
        let len2: usize = s2.len();
        if len1 > len2 {
            return false;
        }
        let chs1: Vec<char> = s1.chars().collect();
        let chs2: Vec<char> = s2.chars().collect();
        let mut freqs: Vec<i32> = {
            let mut freqs: Vec<i32> = vec![0; 26];
            for idx in 0..len1 {
                freqs[chs1[idx] as usize - 'a' as usize] += 1;
                freqs[chs2[idx] as usize - 'a' as usize] -= 1;
            }
            freqs
        };
        if Self::are_all_zeros(&freqs) {
            return true;
        }
        for idx in len1..len2 {
            freqs[chs2[idx] as usize - 'a' as usize] -= 1;
            freqs[chs2[idx - len1] as usize - 'a' as usize] += 1;
            if Self::are_all_zeros(&freqs) {
                return true;
            }
        }
        false
    }
    fn are_all_zeros(nums: &Vec<i32>) -> bool {
        for &num in nums {
            if num != 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let s1: String = "ab".to_owned();
        let s2: String = "eidbaooo".to_owned();
        let actual = Solution::check_inclusion(s1, s2);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let s1: String = "ab".to_owned();
        let s2: String = "eidboaoo".to_owned();
        let actual = Solution::check_inclusion(s1, s2);
        let expected = false;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_075() {
        let s1: String = "adc".to_owned();
        let s2: String = "dcda".to_owned();
        let actual = Solution::check_inclusion(s1, s2);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
