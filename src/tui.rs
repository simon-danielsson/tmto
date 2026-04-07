use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self, MoveTo, MoveToColumn, MoveToNextLine},
    execute,
    style::{Color, SetBackgroundColor},
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
        disable_raw_mode, enable_raw_mode,
    },
};
use figlet_rs::Toilet;

use crate::{Cycle, State, Tmto};

impl Tmto {
    // *brakoll - d: add "fullscreen" command, p: 0, t: feature, s: closed
    pub fn draw_tui(&mut self, time: u64) -> io::Result<()> {
        let mut time_left: u64 = time * 60;
        self.paint_bg()?;

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
                format!(" {label} {elap_min:02}:{time_left:02} ")
            };

            if self.args.big {
                let exp_msg = "Failed to load fonts.";
                let font = Toilet::mono9().expect(exp_msg);
                let prog_text_big =
                    font.convert(&prog_text).expect(exp_msg).as_str();
                self.write_centered_text_big(prog_text_big)?;
            } else {
                let prog_text = self.add_color(&prog_text);
                self.write_centered_text(prog_text)?;
            }

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

    pub fn write_centered_text_big(&mut self, text: String) -> io::Result<()> {
        let lines: Vec<&str> = text.lines().collect();
        let middle_c = self.t_cols / 2;
        let middle_r = self.t_rows / 2;
        let max_width = lines
            .iter()
            .map(|line| line.chars().count() as u16)
            .max()
            .unwrap_or(0);
        let c = middle_c.saturating_sub(max_width as u16 / 2);
        let r = middle_r.saturating_sub(lines.len() as u16 / 2);
        self.sout.queue(MoveTo(c, r))?;
        for l in lines {
            let lc = self.add_color(&l);
            self.sout.write(lc.as_bytes())?;
            self.sout.queue(MoveToNextLine(1))?;
            self.sout.queue(MoveToColumn(c))?;
        }
        Ok(())
    }

    pub fn write_centered_text(&mut self, text: String) -> io::Result<()> {
        let middle_c = self.t_cols / 2;
        let middle_r = self.t_rows / 2;
        let c = middle_c.saturating_sub(text.chars().count() as u16 / 4);
        self.sout.queue(MoveTo(c, middle_r))?;
        self.sout.write(text.as_bytes())?;
        Ok(())
    }

    pub fn tui_resize(&mut self) -> io::Result<()> {
        self.sout.queue(Clear(ClearType::All))?;
        (self.t_cols, self.t_rows) = terminal::size()?;
        self.paint_bg()?;
        Ok(())
    }

    pub fn tui_setup(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        (self.t_cols, self.t_rows) = terminal::size()?;
        self.sout.execute(EnterAlternateScreen)?;
        self.sout.queue(cursor::SavePosition)?;
        self.sout.queue(cursor::Hide)?;
        self.paint_bg()?;
        Ok(())
    }
    pub fn paint_bg(&mut self) -> io::Result<()> {
        let mut bg_col = self.cycle.color();

        if self.state == State::Pause {
            bg_col = self.state.color();
        }
        let txt = format!("{}\x1b[2J", bg_col);

        if self.args.fill {
            write!(self.sout, "{txt}").unwrap();
        }

        Ok(())
    }

    pub fn tui_cleanup(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        self.sout.execute(LeaveAlternateScreen)?;
        self.sout.queue(cursor::RestorePosition)?;
        self.sout.queue(cursor::Show)?;
        Ok(())
    }

    // *brakoll - d: add coloring to text, p: 0, t: feature, s: closed
    fn add_color(&mut self, t: &str) -> String {
        let reset = "\x1b[0m";
        let mut col = self.cycle.color();
        if self.state == State::Pause {
            col = self.state.color();
        }

        format!("{col}{t}{reset}")
    }
}
