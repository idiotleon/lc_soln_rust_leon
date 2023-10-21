//use crate::leetcode::ood::lvl3::lc0341::nested_integer::NestedInteger;
//use std::collections::VecDeque;
//
//struct NestedIterator {
//    stk: VecDeque<NestedInteger>,
//}
//
//impl NestedIterator {
//    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
//        let mut stk: VecDeque<NestedInteger> = VecDeque::new();
//        Self::flatten(&mut stk, &nested_list);
//        NestedIterator { stk }
//    }
//
//    pub fn next(&mut self) -> i32 {
//        if !Self::has_next(self) {
//            return -1;
//        }
//        if let Some(top) = self.stk.pop_back() {
//            match top {
//                NestedInteger::Int(value) => {
//                    return value;
//                }
//
//                NestedInteger::List(_list) => {
//                    unreachable!()
//                }
//            }
//        }
//        return -1;
//    }
//
//    pub fn has_next(&mut self) -> bool {
//        while let Some(top) = self.stk.back_mut() {
//            match top {
//                NestedInteger::Int(_) => {}
//
//                NestedInteger::List(list) => {
//                    Self::flatten(&mut self.stk, list);
//                }
//            }
//        }
//        return !self.stk.is_empty();
//    }
//
//    fn flatten(stk: &mut VecDeque<NestedInteger>, nested_list: &Vec<NestedInteger>) {
//        for &ele in nested_list.into_iter().rev() {
//            // match ele{
//            //     NestedInteger::Int(value) => {
//            //         stk.push_back(NestedInteger::Int(*value))
//            //     }
//            //
//            //     NestedInteger::List(list) => {
//            //         stk.push(NestedInteger::List(*list.clone()))
//            //     }
//            // }
//            stk.push_back(ele.clone())
//        }
//    }
//}
