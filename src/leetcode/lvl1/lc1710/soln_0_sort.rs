/// @author: Leon
/// https://leetcode.com/problems/maximum-units-on-a-truck/
/// Time Complexity:    O(`_len_bs` * lg(`_len_bs`))
/// Space Complexity:   O(`_len_bs`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let _len_bs: usize = box_types.len();
        let sorted: Vec<Vec<i32>> = {
            let mut sorted = box_types;
            sorted.sort_by_key(|e| e[1]);
            sorted
        };
        let mut size_cur: i32 = 0;
        let mut ans: i32 = 0;
        for bt in sorted.iter().rev() {
            let size = bt[0];
            let unit = bt[1];
            if size_cur + size <= truck_size {
                size_cur += size;
                ans += size * unit;
            } else {
                ans += (truck_size - size_cur) * unit;
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let box_types: Vec<Vec<i32>> = vec![vec![1, 3], vec![2, 2], vec![3, 1]];
        let truck_size: i32 = 4;
        let actual = Solution::maximum_units(box_types, truck_size);
        let expected = 8;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let box_types: Vec<Vec<i32>> = vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]];
        let truck_size: i32 = 10;
        let actual = Solution::maximum_units(box_types, truck_size);
        let expected = 91;
        assert_eq!(expected, actual);
    }
}
