/// @author: Leon
/// https://leetcode.com/problems/shuffle-an-array/
/// 
/// Time Complexities:    
///     `shuffle()`:    O(`n`)
///     `reset()`:      O(1)
/// Space Complexity:   O(`n`)
/// 
/// Reference:
/// https://leetcode.com/problems/shuffle-an-array/discuss/1350537/Rust-FisherYates-shuffle
use rand::distributions::Uniform;
use rand::prelude::*;

#[allow(dead_code)]
struct Solution {
    origin: Vec<i32>,
    shuffled: Vec<i32>,
    rng: ThreadRng,
}

#[allow(dead_code)]
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self{
            origin: nums.clone(),
            shuffled: nums,
            rng: thread_rng(),
        }
    }
    
    /** Resets the array to its original configuration and return it. */
    fn reset(&mut self) -> Vec<i32> {
        self.shuffled = self.origin.clone();
        self.origin.clone()
    }
    
    /** Returns a random shuffling of the array. */
    fn shuffle(&mut self) -> Vec<i32> {
        let n = self.shuffled.len();
        for i in 0..(n - 1){
            let uniform = Uniform::from(i..n);
            let j = self.rng.sample(uniform);
            self.shuffled.swap(i, j);
        }
        self.shuffled.clone()
    }
}