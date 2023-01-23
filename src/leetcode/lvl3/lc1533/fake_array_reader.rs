pub struct FakeArrayReader;

impl FakeArrayReader {
    pub fn compare_sub(&self, l: i32, r: i32, x: i32, y: i32) -> i32 {
        // this is a fake/dumy logic
        return l - r + x - y;
    }
    pub fn length(&self) -> i32 {
        // this is a fake/dummy logic
        return 1;
    }
}
