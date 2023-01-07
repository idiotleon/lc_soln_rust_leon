use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let num_to_indices: HashMap<i32, Vec<usize>> = {
            let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
            for (idx, num) in nums.into_iter().enumerate() {
                map.entry(num).or_default().push(idx);
            }
            map
        };
        // to get the largest degree
        let longest: usize = num_to_indices
            .iter()
            .max_by_key(|(_num, indices)| indices.len())
            .unwrap()
            .1
            .len();
        // to get all the indices vectors with the largest degree
        let indices_longest: Vec<Vec<usize>> = num_to_indices
            .into_values()
            .collect::<Vec<Vec<usize>>>()
            .into_iter()
            .filter(|indices| indices.len() == longest)
            .collect();
        let mut shortest: i32 = len_ns as i32;
        for indices in indices_longest {
            let len_idc: usize = indices.len();
            if len_idc == 1 {
                return 1;
            }
            let len: i32 = (indices[len_idc - 1] - indices[0]) as i32 + 1;
            shortest = std::cmp::min(shortest, len);
        }
        return shortest;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let nums: Vec<i32> = vec![1, 2, 2, 3, 1];
        let expected: i32 = 2;
        let actual = Solution::find_shortest_sub_array(nums);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let nums: Vec<i32> = vec![1, 2, 2, 3, 1, 4, 2];
        let expected: i32 = 6;
        let actual = Solution::find_shortest_sub_array(nums);
        assert_eq!(expected, actual);
    }
}
