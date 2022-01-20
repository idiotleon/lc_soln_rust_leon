/// @author: Leon
/// https://leetcode.com/problems/can-place-flowers/
/// Time Complexity:    O(`len_fbs`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/can-place-flowers/discuss/1698771/C%2B%2BJavaPython-2-Simple-Solutions-oror-Image-Explanation-oror-Beginner-Friendly
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let len_fbs: usize = flowerbed.len();
        const FLOWER: i32 = 1;
        const EMPTY: i32 = 0;
        let mut cap: i32 = 0;
        for idx in 0..len_fbs {
            if flowerbed[idx] == EMPTY
                && (idx == 0 || flowerbed[idx - 1] == EMPTY)
                && (idx == len_fbs - 1 || flowerbed[idx + 1] == EMPTY)
            {
                cap += 1;
                if cap >= n {
                    return true;
                }
                flowerbed[idx] = FLOWER;
            }
        }
        n == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        let actual = Solution::can_place_flowers(flowerbed, n);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        let actual = Solution::can_place_flowers(flowerbed, n);
        let expected = false;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_052() {
        let flowerbed = vec![1, 0, 1, 0, 1, 0, 1];
        let n = 2;
        let actual = Solution::can_place_flowers(flowerbed, n);
        let expected = false;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_109() {
        let flowerbed = vec![1, 0, 0, 0, 0, 1];
        let n = 2;
        let actual = Solution::can_place_flowers(flowerbed, n);
        let expected = false;
        assert_eq!(expected, actual);
    }
}
