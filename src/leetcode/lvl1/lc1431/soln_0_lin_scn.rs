/// @author: Leon
/// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
/// Time Complexity:    O(`len_cds`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let len_cds: usize = candies.len();
        let most_candies: i32 = *candies.iter().max().unwrap();
        let mut ans: Vec<bool> = vec![false; len_cds];
        for (idx, cnt) in candies.into_iter().enumerate(){
            ans[idx] = (cnt + extra_candies) >= most_candies;
        } 
        ans
    }
}