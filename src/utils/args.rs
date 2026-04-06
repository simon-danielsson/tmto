use std::io;

#[derive(PartialEq, Clone, Debug)]
pub struct Arguments {
    pub help: bool,
    pub rest: u64, // rest duration in minutes (block - rest = work)
    pub tot: u64,  // total block duration in minutes
}

impl Arguments {
    fn new() -> Self {
        Self {
            help: false,
            rest: u64::default(),
            tot: u64::default(),
        }
    }
}

pub fn parse() -> io::Result<Arguments> {
    let mut a = Arguments::new();
    let mut it = std::env::args().skip(1);

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "help" => {
                a.help = true;
            }

            "-r" => {
                let err_msg = "Invalid rest duration";
                a.rest = it
                    .next()
                    .as_deref()
                    .expect(err_msg)
                    .parse::<u64>()
                    .expect(err_msg);
                }

            "-t" => {
                let err_msg = "Invalid total duration";
                a.tot = it
                    .next()
                    .as_deref()
                    .expect(err_msg)
                    .parse::<u64>()
                    .expect(err_msg);
                }

            _ => {}
        }
    }
    Ok(a)
}
