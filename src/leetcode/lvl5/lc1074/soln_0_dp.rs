/// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/
///
/// Time Complexity:     O(`rows` * (`cols` ^ 2))
/// Space Complexity:    O(`rows` * `cols`)
///
/// Reference:
/// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/discuss/303750/javacpython-find-the-subarray-with-target-sum/910389
/// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/discuss/1162957/Rust-solution
use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_submatrix_sum_target(mut matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let rows = matrix.len() as usize;
        let cols = matrix[0].len() as usize;
        
        for row in 0..rows{
            for col in 1..cols{
                matrix[row][col] += matrix[row][col - 1];
            }
        }
        
        let mut count = 0;
        let mut sum_to_freq = HashMap::new();
        
        for lo in 0..cols{
            for hi in lo..cols{
                sum_to_freq.clear();
                sum_to_freq.insert(0, 1);
                
                let mut sum = 0;
                for row in 0..rows{
                    sum += matrix[row][hi] - if lo > 0 { matrix[row][lo - 1] } else { 0 };
                    let expected = sum - target;
                    
                    if let Some(&freq) = sum_to_freq.get(&expected){
                        count += freq;
                    }
                    
                    // 1
                    // count += sum_to_freq.get(&expected).unwrap_or(&0);
                    
                    // 2
                    // match sum_to_freq.get(&expected){
                    //     Some(&freq) => count += freq,
                    //     None => ()
                    // }
                    
                    // 3
                    // count += if sum_to_freq.contains_key(&expected) { sum_to_freq[&expected] } else { 0 };

                    *sum_to_freq.entry(sum).or_insert(0) += 1;
                }
            }
        }
        
        count
    }
}