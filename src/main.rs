use std::{
    env,
    error::Error,
    io::{stdin, stdout, BufRead, Write},
    time::{Duration, Instant},
};

struct Opts {
    rate: f32,
    period: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = parse_opts()?;
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

fn parse_opts() -> Result<Opts, Box<dyn Error>> {
    let mut rate_opt = None;
    let mut period_opt = None;
    let mut args = env::args().skip(1);

    loop {
        match args.next().as_deref() {
            Some(opt @ "-r" | opt @ "--rate") => rate_opt.replace(
                args.next()
                    .ok_or_else(|| format!("Missing required argument to {}", opt))?,
            ),
            Some(opt @ "-p" | opt @ "--period") => period_opt.replace(
                args.next()
                    .ok_or_else(|| format!("Missing required argument to {}", opt))?,
            ),
            Some(opt) => return Err(format!("Unknown option {}", opt).into()),
            _ => break,
        };
    }

    let rate = rate_opt.unwrap_or_else(|| "1.0".to_string()).parse()?;
    let period = period_opt.unwrap_or_else(|| "1s".to_string());

    Ok(Opts { rate, period })
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
