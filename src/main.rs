use std::{
    error::Error,
    io::{stdin, stdout, BufRead, Write},
    time::Instant,
};

use clap::{ColorChoice::*, Parser};
use duration_str::parse as parse_duration;

#[derive(Parser, Debug)]
#[clap(version, about, color = Never)]
struct Opts {
    #[clap(
        short,
        long,
        default_value = "1",
        help = "Output rate in lines per period (-p)"
    )]
    rate: f32,

    #[clap(
        short,
        long,
        default_value = "1s",
        help = "Time period to apply output rate (-r) to"
    )]
    period: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let tick = parse_duration(&opts.period)?.div_f32(opts.rate);
    let mut output = stdout().lock();
    let mut t0 = Instant::now();

    for line in stdin().lock().lines() {
        if t0.elapsed() >= tick {
            t0 = Instant::now();

            output.write_all(line?.as_bytes())?;
            output.write_all(b"\n")?;
            output.flush()?;
        }
    }

    Ok(())
}
