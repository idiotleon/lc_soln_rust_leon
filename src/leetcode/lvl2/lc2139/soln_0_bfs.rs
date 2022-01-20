/// @author: Leon
/// https://leetcode.com/problems/minimum-moves-to-reach-target-score/
/// Time Complexity:    O(`target`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_moves(target: i32, max_doubles: i32) -> i32 {
        let mut steps: i32 = 0;
        let mut tgt = target;
        let mut doubles_used = 0;
        while tgt > 1 {
            if tgt % 2 == 0 {
                if doubles_used < max_doubles {
                    tgt /= 2;
                    doubles_used += 1;
                } else {
                    return steps + (tgt - 1);
                }
            } else {
                tgt -= 1;
            }
            steps += 1;
        }
        steps
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let target: i32 = 5;
        let max_doubles: i32 = 0;
        let actual = Solution::min_moves(target, max_doubles);
        let expected = 4;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let target: i32 = 19;
        let max_doubles: i32 = 2;
        let actual = Solution::min_moves(target, max_doubles);
        let expected = 7;
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_works_with_sample_input_3() {
        let target: i32 = 10;
        let max_doubles: i32 = 4;
        let actual = Solution::min_moves(target, max_doubles);
        let expected = 4;
        assert_eq!(expected, actual);
    }
}
