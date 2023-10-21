#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

// #[derive(Debug, Eq, PartialEq)]
// pub struct NestedInteger {
//     pub integer: i32,
//     pub list: Vec<NestedInteger>,
// }
//
// impl NestedInteger {
//     pub fn is_integer() -> bool{
//         true
//     }
//     pub fn get_integer() -> i32{
//         1
//     }
//     pub fn add(nested_integer: NestedInteger){
//
//     }
//     pub fn get_list() -> Vec<i32>{
//         vec![0]
//     }
// }
