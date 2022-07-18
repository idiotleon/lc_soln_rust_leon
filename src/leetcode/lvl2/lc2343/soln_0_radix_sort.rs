// use std::collections::HashMap;

/// @author: Leon
/// Time Complexity:    O(`len_qs` * `len_ns` * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/query-kth-smallest-trimmed-number/discuss/2296267/C%2B%2B-Radix-Sort
/// https://leetcode.com/problems/query-kth-smallest-trimmed-number/discuss/2293653/C%2B%2B-or-Radix-Sort-only-once-or-O(nd-%2B-q)
/// https://leetcode.com/problems/query-kth-smallest-trimmed-number/discuss/2292752/Python-Radix-Sort-O(k-*-(n-%2B-10)-%2B-q)
/// https://leetcode.com/problems/query-kth-smallest-trimmed-number/discuss/2294533/C%2B%2B-or-O(MN-%2B-Q)-or-Very-Detailed-Optimal-Radix-Sort-Based-Solution
/// Note:
/// this is not yet a correct solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    // pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    //     let len_ss: usize = nums.len();
    //     let len_qs: usize = queries.len();
    //     let str_to_idx: HashMap<String, usize> = {
    //         let mut map: HashMap<String, usize> = HashMap::with_capacity(len_ss);
    //         for (idx, num) in nums.iter().enumerate(){
    //             map.insert(num.to_owned(), idx);
    //         }
    //         map
    //     };
    //     let mut ans: Vec<i32> = Vec::with_capacity(len_qs);
    //     for query in queries {
    //         let k: usize = query[0] as usize;
    //         let trim: usize = query[1] as usize;
    //         let res = Self::get_kth_num(&nums, trim, k);
    //         ans.push(res);
    //     }
    //     ans
    // }
    // fn get_kth_num(nums: &Vec<String>, trim: usize, k: usize) -> i32 {
    //     let len_ns: usize = nums.len();
    //     let mut res: Vec<(&str, usize)> = Vec::with_capacity(len_ns);
    //     for (idx, num) in nums.iter().enumerate() {
    //         let len_n: usize = num.len();
    //         res.push((&num[len_n - trim..], idx));
    //     }
    //     res.sort_by_key(|&r| r.0);
    //     res[k - 1].1 as i32
    // }
    // fn radix_sort(nums: &mut Vec<Vec<char>>, len: usize){
    // }
    // fn counting_sort(nums: &mut Vec<Vec<char>>, len: usize, idx_digit: usize){
    // }
    // fn radix_sort1(nums: &mut Vec<(i32, usize)>, len: usize){
    //     let max_subarray = Self::get_subarray_max(nums, len);
    //     let mut place: i32 = 1;
    //     while max_subarray / place > 0{
    //         Self::counting_sort1(nums, len, place);
    //         place *= 10;
    //     }
    // }
    // fn counting_sort1(nums: &mut Vec<(i32, usize)>, len: usize, place: i32){
    //     let _len_ns: usize = nums.len();
    //     let mut output: Vec<(i32, usize)> = vec![(0, 0); len];
    //     let max_subarray = *nums[..len].iter().map(|(num, _idx)| num).max().unwrap();
    //     let mut freqs: Vec<i32> = {
    //         let mut freqs: Vec<i32> = vec![0; max_subarray as usize + 1];
    //         // to get the frequences/counts of each number
    //         for idx in 0..len{
    //             freqs[(nums[idx].0 / place) as usize % 10] += 1;
    //         }
    //         // to calculate the cumulative frequences/counts for digits 0 to 10
    //         for digit in 1..10{
    //             freqs[digit] += freqs[digit - 1];
    //         }
    //         freqs
    //     };
    //     // to place the elements in the sorted order
    //     for idx in (0..len).rev(){
    //         let idx_digit: usize = (nums[idx].0 / place) as usize % 10;
    //         output[freqs[idx_digit] as usize - 1] = nums[idx];
    //         freqs[idx_digit] -= 1;
    //     }
    //     for idx in 0..len{
    //         nums[idx] = output[idx];
    //     }
    // }
    // fn get_subarray_max(nums: &Vec<(i32, usize)>, len: usize) -> i32{
    //     nums[..len].iter().max().unwrap().0
    // }
}
