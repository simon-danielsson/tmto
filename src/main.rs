use std::{
    io::{self, Stdout, Write, stdout},
    thread,
    time::Duration,
};

use crate::utils::args::Arguments;

mod help;
mod utils;

fn main() -> io::Result<()> {
    let mut t = Tmto::new(utils::args::parse()?)?;

    if t.args.help {
        help::print_help();
        return Ok(());
    }

    // *brakoll - d: add check for if arguments have been supplied, p: 0, t: feature, s: closed
    if t.args.tot == 0 || t.args.rest == 0 {
        println!(
            "You did not supply the correct flags! Use \"tmto help\" if you're feeling stuck."
        );
        return Ok(());
    }

    // *brakoll - d: remove debug print msg, p: 0, t: fix, s: closed
    // t.info_print();

    while t.state == State::Active {
        match t.cycle {
            Cycle::Work => t.draw(t.args.tot - t.args.rest)?,
            Cycle::Rest => t.draw(t.args.rest)?,
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
enum State {
    Active,
    Quit,
}

#[derive(Debug, PartialEq)]
enum Cycle {
    Work,
    Rest,
}

struct Tmto {
    args: Arguments,
    sout: Stdout,
    cycle: Cycle,
    state: State,
}

impl Tmto {
    fn new(args: Arguments) -> io::Result<Self> {
        Ok(Self {
            args,
            sout: stdout(),
            cycle: Cycle::Work,
            state: State::Active,
        })
    }

    fn info_print(&mut self) {
        let txt = format!(
            "settings - total: {t}, work: {w}, rest: {r}",
            t = self.args.tot,
            w = (self.args.tot - self.args.rest),
            r = self.args.rest
        );
        println!("{}\n", txt);
    }

    fn move_cursor_up(&mut self, n: usize) {
        print!("\x1b[{}A", n);
    }

    fn clear_line(&mut self, t: &str, newline: bool) {
        if newline {
            print!("\r\x1b[2K{}\n", t);
        } else {
            print!("\r\x1b[2K{}", t);
        }
    }

    fn draw(&mut self, time: u64) -> io::Result<()> {
        for elap_sec in (0..=time * 60).rev() {
            // reset cursor
            self.move_cursor_up(2);

            {
                let c_cyc_txt = match self.cycle {
                    Cycle::Work => "WORK",
                    Cycle::Rest => "REST",
                };

                let elap_min = elap_sec / 60;
                let elap_sec = elap_sec % 60;
                let prog_text =
                    format!("\n{c_cyc_txt} [{elap_min:02}:{elap_sec:02}]\n");

                self.clear_line(&prog_text, false);
            }

            self.sout.flush()?;
            // *brakoll - d: change millis to sec, p: 0, t: fix, s: closed
            thread::sleep(Duration::from_secs(1));
        }
        if self.cycle == Cycle::Rest {
            self.cycle = Cycle::Work
        } else {
            self.cycle = Cycle::Rest
        }

        Ok(())
    }
}