pub trait Input {
    fn to_string(&self) -> &[u8];

    fn char(&mut self, index: usize, c: i32);
}
