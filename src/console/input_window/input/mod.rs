mod ipv4;
mod string;
mod u8;

pub use ipv4::IPv4Input;
pub use string::StringInput;
pub use u8::U8Input;

pub trait Input {
    fn label(&self) -> &str;

    fn text(&self) -> &[u8];
    fn index(&self) -> usize;

    fn backspace(&mut self);
    fn char(&mut self, c: u8);
}
