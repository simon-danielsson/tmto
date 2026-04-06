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

    // *brakoll - d: hide cursor, p: 0, t: feature, s: closed
    // hide cursor
    {
        ctrlc::set_handler(|| {
            print!("\x1b[?25h");
            let _ = stdout().flush();
            std::process::exit(130);
        })
        .expect("Failed to set Ctrl-C handler.");
        print!("\x1b[?25l");
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

    // *brakoll - d: add coloring to text, p: 0, t: feature, s: closed
    fn add_color(&self, t: &str) -> String {
        let reset = "\x1b[0m";
        let col = match self.cycle {
            Cycle::Work => "\x1b[30;44m", // black on blue
            _ => "\x1b[30;42m",           // black on green
        };
        format!("{col}{t}{reset}")
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

    // *brakoll - d: make sure that the line is cleared properly before cycle starts, p: 0, t: fix, s: closed
    fn draw(&mut self, time: u64) -> io::Result<()> {
        for elap_sec in (0..=time * 60).rev() {
            // reset cursor
            self.move_cursor_up(1);

            {
                let c_cyc_txt = match self.cycle {
                    Cycle::Work => "WORK",
                    Cycle::Rest => "REST",
                };

                let elap_min = elap_sec / 60;
                let elap_sec = elap_sec % 60;
                let mut prog_text =
                    format!(" {c_cyc_txt} {elap_min:02}:{elap_sec:02} \n");
                prog_text = self.add_color(&prog_text);

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