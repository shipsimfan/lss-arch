use super::{curses::CHType, error::CursesResult, window::Window, Console};

pub struct SelectWindow<'a> {
    window: Window<'a>,

    values: Vec<String>,
    current_option: usize,

    x: i32,

    selected_color: CHType,
}

impl<'a> SelectWindow<'a> {
    pub fn run(
        console: &'a mut Console,
        title: &str,
        message: &str,
        values: Vec<String>,
    ) -> CursesResult<usize> {
        let mut window = SelectWindow::new(console, title, message, values)?;

        loop {
            let c = window.get_char()?;

            match c {
                x if x == b'\n' as i32 => return Ok(window.current_option),
                65 => window.up()?,
                66 => window.down()?,
                _ => {}
            }
        }
    }

    fn new(
        console: &'a mut Console,
        title: &str,
        message: &str,
        values: Vec<String>,
    ) -> CursesResult<Self> {
        assert!(values.len() > 0);

        let mut longest_value = 0;
        for value in &values {
            if value.len() > longest_value {
                longest_value = value.len();
            }
        }

        let width = (message.len() + 2).max(longest_value + 12) as i32;
        let x = (width / 2) - (longest_value as i32 / 2);

        let selected_color = console.colors().background_color();
        let mut window = Window::new(console, width, values.len() as i32 + 6, title)?;
        window.write_at(1, 1, message.as_bytes())?;

        window.write_at_with_attribute(
            (width / 2) - 4,
            values.len() as i32 + 4,
            selected_color,
            "<  OK  >".as_bytes(),
        )?;

        let mut window = SelectWindow {
            window,

            values,
            current_option: 0,

            x,

            selected_color,
        };

        for i in 0..window.values.len() {
            window.write_option(i)?;
        }
        window.window.refresh()?;

        Ok(window)
    }

    fn get_char(&mut self) -> CursesResult<i32> {
        self.window.get_char()
    }

    fn up(&mut self) -> CursesResult<()> {
        if self.current_option == 0 {
            return Ok(());
        }

        self.current_option -= 1;
        self.write_option(self.current_option)?;
        self.write_option(self.current_option + 1)?;
        self.window.refresh()
    }

    fn down(&mut self) -> CursesResult<()> {
        if self.current_option == self.values.len() - 1 {
            return Ok(());
        }

        self.current_option += 1;
        self.write_option(self.current_option)?;
        self.write_option(self.current_option - 1)?;
        self.window.refresh()
    }

    fn write_option(&mut self, option: usize) -> CursesResult<()> {
        let y = 3 + option as i32;

        self.window.write_at_with_attribute(
            self.x,
            y,
            if self.current_option == option {
                self.selected_color
            } else {
                0
            },
            self.values[option].as_bytes(),
        )
    }
}
