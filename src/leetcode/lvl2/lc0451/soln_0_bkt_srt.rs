// use std::collections::HashMap;

// #[allow(dead_code)]
// pub struct Solution;

// #[allow(dead_code)]
// impl Solution {
//     pub fn frequency_sort(s: String) -> String {
//         let len = s.len();
//         let ch_to_freq: HashMap<char, u32> = {
//             let mut tmp: HashMap<char, u32> = HashMap::new();
//             for ch in s.chars() {
//                 let freq = tmp.get(&ch).unwrap_or(0);
//                 tmp.insert(ch, 1 + freq);
//             }
//             tmp
//         };

//         let bkts: Vec<Vec<char>> = {
//             let mut tmp = vec![Vec::<char>::new(); len];
//             for (ch, val) in &ch_to_freq {
//                 tmp[*val as usize].push(*ch);
//             }
//             tmp
//         };

//         let ans: &str = {
//             let tmp = &"";

//             for lst in bkts.iter().rev() {
//                 if !lst.is_empty() {
//                     for ch in lst{

//                     }
//                 }
//             }
//         };

//         ans.to_string()
//     }
// }
