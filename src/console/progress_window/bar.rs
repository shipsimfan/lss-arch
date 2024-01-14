use crate::console::{
    curses::{self, CHType},
    error::CursesResult,
    window::Window,
};

pub(super) struct ProgressBar {
    window: curses::Window,
    bar_color: CHType,

    width: i32,

    current_step: i32,
    total_steps: i32,
}

fn calculate_percentage(current_step: i32, total_steps: i32) -> i32 {
    ((current_step * 100) / total_steps).clamp(0, 100)
}

fn write_percentage(
    window: curses::Window,
    width: i32,
    percentage: i32,
    bar_end: i32,
    bar_color: CHType,
) -> CursesResult<()> {
    let string = format!("{}%", percentage).into_bytes();
    let x = (width / 2) - (string.len() as i32 / 2);

    curses::wmove(window, x, 1)?;

    let mut attribute_active = false;
    for i in 0..string.len() as i32 {
        if !attribute_active {
            if x + i < bar_end {
                attribute_active = true;
                curses::wattron(window, bar_color)?;
            }
        } else {
            if x + i >= bar_end {
                attribute_active = false;
                curses::wattroff(window, bar_color)?;
            }
        }

        curses::waddch(window, string[i as usize] as char)?;
    }

    Ok(())
}

impl ProgressBar {
    pub(super) fn new(parent: &mut Window, total_steps: i32) -> CursesResult<Self> {
        let width = parent.width() - 4;
        let bar_color = parent.console().colors().background_color();

        let window = curses::derwin(parent.inner(), 2, 3, width, 3)?;
        curses::touchwin(parent.inner())?;

        curses::r#box(window, 0, 0)?;
        write_percentage(window, width, 0, 0, bar_color)?;

        curses::wrefresh(window)?;
        Ok(ProgressBar {
            window,
            bar_color,

            width,

            current_step: 0,
            total_steps,
        })
    }

    pub fn step(&mut self) -> CursesResult<()> {
        let percentage = calculate_percentage(self.current_step, self.total_steps);
        let bar_end = (((self.width - 2) * percentage) / 100) + 1;

        curses::wattron(self.window, self.bar_color)?;
        curses::wmove(self.window, 1, 1)?;
        for _ in 1..bar_end {
            curses::waddch(self.window, ' ')?;
        }
        curses::wattroff(self.window, self.bar_color)?;

        write_percentage(self.window, self.width, percentage, bar_end, self.bar_color)?;

        curses::wrefresh(self.window)?;

        self.current_step += 1;
        Ok(())
    }
}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        curses::delwin(self.window).ok();
    }
}
