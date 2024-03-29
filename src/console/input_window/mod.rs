use super::{error::CursesResult, window::Window, Console};
use curses::{KEY_BACKSPACE, KEY_DOWN, KEY_ENTER, KEY_UP};
use input::Input;

mod input;

pub use input::{IPv4Input, StringInput, U8Input};

pub struct InputWindow<'a> {
    window: Window<'a>,
    width: i32,
    selected_color: curses::CHType,

    inputs: &'a mut [&'a mut dyn Input],
    index: usize,
}

fn is_valid_char(c: i32) -> bool {
    if c < 0 || c > u8::MAX as i32 {
        return false;
    }
    (c as u8).is_ascii_graphic() || (c as u8) == b' '
}

impl<'a> InputWindow<'a> {
    pub fn run(
        console: &'a mut Console,
        title: &str,
        message: &str,
        inputs: &'a mut [&'a mut dyn Input],
    ) -> CursesResult<()> {
        let mut window = InputWindow::new(console, title, message, inputs)?;

        loop {
            let c = window.get_char()?;

            match c {
                KEY_ENTER | 10 => return Ok(()),
                x if is_valid_char(x) => window.char(c as u8)?,
                KEY_UP => window.up()?,
                KEY_DOWN => window.down()?,
                KEY_BACKSPACE => window.backspace()?,
                _ => {}
            }
        }
    }

    fn new(
        console: &'a mut Console,
        title: &str,
        message: &str,
        inputs: &'a mut [&'a mut dyn Input],
    ) -> CursesResult<Self> {
        assert!(inputs.len() > 0);

        let mut width = message.len() as i32;
        for input in inputs.iter() {
            let half_width = input.label().len().max(input.text().len()) + 6;
            width = width.max(half_width as i32 * 2);
        }
        let width = width + 2;

        let selected_color = console.colors().background_color();
        let mut window = Window::new(console, width, inputs.len() as i32 + 6, title)?;
        window.write_at(1, 1, message.as_bytes())?;

        window.write_at_with_attribute(
            (width / 2) - 4,
            inputs.len() as i32 + 4,
            selected_color,
            "<  OK  >".as_bytes(),
        )?;

        let mut window = InputWindow {
            window,
            width,
            selected_color,

            inputs,
            index: 0,
        };

        for i in 0..window.inputs.len() {
            window.window.write_at(
                (width / 2) - 1 - window.inputs[i].label().len() as i32,
                i as i32 + 3,
                window.inputs[i].label().as_bytes(),
            )?;
            window.window.write(b": ")?;
            window.write_option(i)?;
        }
        window.window.refresh()?;

        Ok(window)
    }

    fn write_option(&mut self, option: usize) -> CursesResult<()> {
        let x = (self.width / 2) + 1;
        let bytes = self.inputs[option].text();

        self.window.write_at(x, option as i32 + 3, bytes)?;

        if option == self.index {
            let index = self.inputs[option].index();
            self.window.write_at_with_attribute(
                x + index as i32,
                option as i32 + 3,
                self.selected_color,
                &[bytes[index]],
            )?;
        }

        Ok(())
    }

    fn char(&mut self, char: u8) -> CursesResult<()> {
        self.inputs[self.index].char(char);
        self.write_option(self.index)
    }

    fn backspace(&mut self) -> CursesResult<()> {
        self.inputs[self.index].backspace();
        self.write_option(self.index)
    }

    fn up(&mut self) -> CursesResult<()> {
        if self.index == 0 {
            return Ok(());
        }

        self.index -= 1;
        self.write_option(self.index)?;
        self.write_option(self.index + 1)?;
        self.window.refresh()
    }

    fn down(&mut self) -> CursesResult<()> {
        if self.index == self.inputs.len() - 1 {
            return Ok(());
        }

        self.index += 1;
        self.write_option(self.index)?;
        self.write_option(self.index - 1)?;
        self.window.refresh()
    }

    fn get_char(&mut self) -> CursesResult<i32> {
        self.window.get_char()
    }
}
