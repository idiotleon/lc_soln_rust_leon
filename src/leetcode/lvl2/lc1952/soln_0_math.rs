/// https://leetcode.com/problems/three-divisors/
/// 
/// Time Complexity:    O(sqrt(`n`))
/// Space Complexity:   O(1)
/// 
/// Reference:
/// https://leetcode.com/problems/three-divisors/discuss/1375566/Square-of-Prime-O(1)/1031671
/// https://leetcode.com/problems/three-divisors/discuss/1375566/Square-of-Prime-O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut count: u8 = 0;
        let mut num: i32 = 1;
        
        while num * num <= n && count <= 3{
            if n % num == 0{
                if num * num == n {
                    count += 1;
                }else{
                    count += 2;
                }
            }
            
            num += 1;
        }
        
        count == 3
    }
}