/// @author: Leon
/// https://leetcode.com/problems/truncate-sentence/
/// 
/// Time Complexity:  O(L)
/// Space Complexity: O(1)
impl Solution {
    pub fn truncate_sentence(str: String, k: i32) -> String {
        let mut cnt: i32 = 0;
        
        let mut ans = String::new();
        
        for ch in str.chars(){
            if ch == ' '{
                cnt += 1;
                if cnt == k{
                    break;
                }
            }
            
            ans.push(ch);
        }
        
        ans
    }
}