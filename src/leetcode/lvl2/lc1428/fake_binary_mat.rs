pub struct FakeBinaryMatrix;

impl FakeBinaryMatrix {
    pub fn get(&self, row: i32, col: i32) -> i32 {
        return row + col;
    }
    pub fn dimensions(&self) -> Vec<i32> {
        return vec![8, 24];
    }
}
