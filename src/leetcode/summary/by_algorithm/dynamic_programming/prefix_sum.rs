use std::collections::HashMap;
/// @author: Leon
/// Summary of operaitons obout prefix sums
struct Summary;

#[allow(dead_code)]
impl Summary {
    /// to build up the prefix sums array, with 0-indexed
    fn build_prefix_sums_1(nums: &Vec<i32>) -> Vec<i32> {
        let len_n = nums.len();
        let mut prefix_sums: Vec<i32> = vec![0; len_n];
        for (idx, &num) in nums.into_iter().enumerate() {
            if idx == 0 {
                prefix_sums[idx] = num;
            } else {
                prefix_sums[idx] = prefix_sums[idx - 1] + num;
            }
        }
        prefix_sums
    }
    //TODO: to get the prefix sums of the subarray, with 0-indexed
    // with care of array index out of bound exception

    /// to build up the prefix sums array, with 1-indexed
    fn build_prefix_sums_2(nums: &Vec<i32>) -> Vec<i32> {
        let len_n = nums.len();
        let mut prefix_sums: Vec<i32> = vec![0; len_n + 1];
        for (idx, num) in nums.into_iter().enumerate() {
            prefix_sums[1 + idx] = prefix_sums[idx] + num;
        }
        prefix_sums
    }
    //TODO: to get the prefix sums of the subarray, with 1-indexed
    // with care of array index out of bound exception

    /// to build the prefix sums hashmap with indices
    fn build_prefix_sums_with_idx(nums: &Vec<i32>) -> HashMap<i32, i32> {
        let len_n: usize = nums.len();
        let mut prefix_sums_to_idx: HashMap<i32, i32> = {
            let mut map: HashMap<i32, i32> = HashMap::with_capacity(len_n);
            map.insert(0, -1);
            map
        };
        let mut sum: i32 = 0;
        for (idx, num) in nums.into_iter().enumerate() {
            sum += num;
            prefix_sums_to_idx.insert(sum, idx as i32);
        }
        prefix_sums_to_idx
    }
}
