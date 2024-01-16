use super::{error::CursesResult, Console};
use curses::KEY_ENTER;

pub struct MessageWindow;

impl MessageWindow {
    pub fn run(console: &mut Console, title: &str, message: &[String]) -> CursesResult<()> {
        let mut longest_line = 0;
        for line in message {
            if line.len() as i32 > longest_line {
                longest_line = line.len() as i32;
            }
        }

        let button_color = console.colors().background_color();
        let mut window = console.new_window(longest_line + 4, message.len() as i32 + 4, title)?;

        for i in 0..message.len() {
            window.write_at(2, 1 + i as i32, message[i].as_bytes())?;
        }

        window.write_at_with_attribute(
            longest_line / 2 - 4 + 2,
            message.len() as i32 + 2,
            button_color,
            "<  OK  >".as_bytes(),
        )?;

        loop {
            let c = window.get_char()?;
            if c == KEY_ENTER || c == 10 {
                break;
            }
        }

        Ok(())
    }
}
