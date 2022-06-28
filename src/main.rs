use std::{
    error::Error,
    io::{stdin, stdout, BufRead, Write},
    time::{Duration, Instant},
};

use clap::{ColorChoice::*, Parser};

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
    let tick = parse_duration(opts.period)?.div_f32(opts.rate);
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

fn parse_duration(s: String) -> Result<Duration, Box<dyn Error>> {
    let s = s.to_ascii_lowercase();
    let ds: String = s
        .chars()
        .take_while(|ch| !ch.is_alphabetic() && !ch.is_ascii_whitespace())
        .collect();
    let us: String = s.chars().skip_while(|ch| !ch.is_alphabetic()).collect();
    let n: u64 = ds.parse()?;

    match us.as_str() {
        "usec" | "us" | "µs" => Ok(Duration::from_micros(n)),
        "msec" | "ms" => Ok(Duration::from_millis(n)),
        "seconds" | "second" | "sec" | "s" => Ok(Duration::from_secs(n)),
        "minutes" | "minute" | "min" | "m" => Ok(Duration::from_secs(n * 60)),
        "hours" | "hour" | "hr" | "h" => Ok(Duration::from_secs(n * 60 * 60)),
        _ => Err(format!("Invalid duration unit '{}'", us).into()),
    }
}
