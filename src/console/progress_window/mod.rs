use self::bar::ProgressBar;
use super::{error::CursesResult, window::Window, Console};

mod bar;

pub struct ProgressWindow<'window> {
    progress_bar: ProgressBar,
    window: Window<'window>,

    blank: Vec<u8>,
}

impl<'window> ProgressWindow<'window> {
    pub fn new(console: &'window mut Console, total_steps: i32, title: &str) -> CursesResult<Self> {
        let width = (console.width() * 4) / 5;

        let mut window = console.new_window(width, 7, title)?;
        let progress_bar = ProgressBar::new(&mut window, total_steps)?;

        let blank = vec![b' '; width as usize - 3];

        Ok(ProgressWindow {
            progress_bar,
            window,
            blank,
        })
    }

    pub fn step(&mut self, message: &str) -> CursesResult<()> {
        self.window.write_at(2, 1, &self.blank)?;
        self.window.write_at(2, 1, message.as_bytes())?;

        self.progress_bar.step()?;

        self.window.refresh()
    }
}
