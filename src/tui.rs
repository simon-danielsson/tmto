use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self, MoveTo},
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
        disable_raw_mode, enable_raw_mode,
    },
};

use crate::{Cycle, State, Tmto};

impl Tmto {
    // *brakoll - d: add "fullscreen" command, p: 0, t: feature, s: closed
    pub fn draw_tui(&mut self, time: u64) -> io::Result<()> {
        self.tui_setup()?;

        let mut time_left: u64 = time * 60;

        while time_left > 0 {
            let mut label = match self.cycle {
                Cycle::Work => "WORK",
                Cycle::Rest => "REST",
            };
            self.controls()?;

            match self.state {
                State::Quit => {
                    return Ok(());
                }
                State::Active => {}
                State::Pause => label = "PAUSE",
            }

            // *brakoll - d: less code in label logic in tui, p: 0, t: refactor, s: closed
            let prog_text = {
                let elap_min = time_left / 60;
                let time_left = time_left % 60;
                let label = format!(" {label} {elap_min:02}:{time_left:02} ");
                self.add_color(&label)
            };

            self.write_centered_text(prog_text)?;

            self.sout.flush()?;
            // *brakoll - d: change millis to sec, p: 0, t: fix, s: closed
            if self.state != State::Pause {
                time_left -= 1;
                thread::sleep(Duration::from_secs(1));
            }

            // adjust if term changes size
            if (self.t_cols, self.t_rows) != terminal::size()? {
                self.tui_resize()?;
            }
        }

        Ok(())
    }

    pub fn write_centered_text(&mut self, text: String) -> io::Result<()> {
        let middle_c = self.t_cols / 2;
        let middle_r = self.t_rows / 2;
        let c = middle_c - (text.chars().count() / 4) as u16;
        self.sout.queue(MoveTo(c, middle_r))?;
        self.sout.write(text.as_bytes())?;
        Ok(())
    }

    pub fn tui_resize(&mut self) -> io::Result<()> {
        self.sout.queue(Clear(ClearType::All))?;
        (self.t_cols, self.t_rows) = terminal::size()?;
        Ok(())
    }

    pub fn tui_setup(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        (self.t_cols, self.t_rows) = terminal::size()?;
        self.sout.execute(EnterAlternateScreen)?;
        self.sout.queue(cursor::SavePosition)?;
        self.sout.queue(cursor::Hide)?;
        Ok(())
    }

    pub fn tui_cleanup(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        self.sout.execute(LeaveAlternateScreen)?;
        self.sout.queue(cursor::RestorePosition)?;
        self.sout.queue(cursor::Show)?;
        Ok(())
    }
}