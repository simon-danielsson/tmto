use std::io::{self, Stdout, stdout};

use crossterm::terminal;

use crate::utils::args::Arguments;

mod controls;
mod help;
mod tui;
mod utils;

// *brakoll - d: add pause key (perhaps 'p'?) to pause timer, p: 0, t: feature, s: closed
// *brakoll - d: add queue key (perhaps 'q'?) to queue exiting the app after the current cycle ends, p: 0, t: feature, s: open

fn main() -> io::Result<()> {
    let term_size = terminal::size()?;
    let mut t = Tmto::new(utils::args::parse()?, term_size.0, term_size.1)?;

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

    while t.state != State::Quit {
        match t.cycle {
            Cycle::Work => t.draw_tui(t.args.tot - t.args.rest)?,
            Cycle::Rest => t.draw_tui(t.args.rest)?,
        }
        // cycle intervals
        if t.cycle == Cycle::Rest {
            t.cycle = Cycle::Work
        } else {
            t.cycle = Cycle::Rest
        }
    }
    t.tui_cleanup()?;

    Ok(())
}

#[derive(Debug, PartialEq)]
enum State {
    Active,
    Pause,
    Quit,
}

#[derive(Debug, PartialEq)]
enum Cycle {
    Work,
    Rest,
}

struct Tmto {
    args: Arguments,
    t_cols: u16,
    t_rows: u16,
    sout: Stdout,
    cycle: Cycle,
    state: State,
}

impl Tmto {
    fn new(args: Arguments, t_cols: u16, t_rows: u16) -> io::Result<Self> {
        Ok(Self {
            args,
            t_cols,
            t_rows,
            sout: stdout(),
            cycle: Cycle::Work,
            state: State::Active,
        })
    }

    // *brakoll - d: add coloring to text, p: 0, t: feature, s: closed
    fn add_color(&self, t: &str) -> String {
        let reset = "\x1b[0m";
        let mut col = match self.cycle {
            Cycle::Work => "\x1b[30;44m", // black on blue
            _ => "\x1b[30;42m",           // black on green
        };
        if self.state == State::Pause {
            col = "\x1b[30;41m"; // black on blue
        }

        format!("{col}{t}{reset}")
    }
}