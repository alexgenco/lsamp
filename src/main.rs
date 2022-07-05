use std::{
    env,
    error::Error,
    io::{stdin, stdout, BufRead, Write},
    process::exit,
    time::{Duration, Instant},
};

const USAGE: &str = r#"Usage: lsamp [options]

Options:
    -r, --rate N    Output rate in lines per period [default: 1]
    -p, --period P  Time period to apply output rate to [default: 1s]
    -h, --help      Display help
"#;

struct Opts {
    rate: f32,
    period: Duration,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = match parse_opts() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("lsamp error: {}", e);
            exit(1);
        }
    };
    let tick = opts.period.div_f32(opts.rate);
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
            Some(opt @ "-r" | opt @ "--rate") => {
                let val = args
                    .next()
                    .ok_or_else(|| format!("{}: argument required", opt))?;

                rate_opt.replace(
                    val.parse()
                        .map_err(|_| format!("{}: invalid int {:?}", opt, val))?,
                );
            }
            Some(opt @ "-p" | opt @ "--period") => {
                let val = args
                    .next()
                    .ok_or_else(|| format!("{}: argument required", opt))?;

                period_opt.replace(parse_duration(val, opt)?);
            }
            Some("-h" | "--help") => {
                print!("{}", USAGE);
                exit(0);
            }
            Some(opt) => {
                return Err(format!("unknown option {}", opt).into());
            }
            _ => break,
        };
    }

    let rate = rate_opt.unwrap_or(1.0);
    let period = period_opt.unwrap_or(Duration::from_secs(1));

    Ok(Opts { rate, period })
}

fn parse_duration(s: String, opt: &str) -> Result<Duration, Box<dyn Error>> {
    let s: String = s.to_ascii_lowercase().split_ascii_whitespace().collect();
    let n: u64 = s
        .chars()
        .take_while(|ch| ch.is_numeric())
        .collect::<String>()
        .parse()
        .map_err(|_| format!("{}: invalid duration", opt))?;

    let us: String = s
        .chars()
        .skip_while(|ch| ch.is_numeric())
        .take_while(|ch| ch.is_alphabetic())
        .collect();

    match us.as_str() {
        "usec" | "us" | "µs" => Ok(Duration::from_micros(n)),
        "msec" | "ms" => Ok(Duration::from_millis(n)),
        "seconds" | "second" | "sec" | "s" => Ok(Duration::from_secs(n)),
        "minutes" | "minute" | "min" | "m" => Ok(Duration::from_secs(n * 60)),
        "hours" | "hour" | "hr" | "h" => Ok(Duration::from_secs(n * 60 * 60)),
        "" => Err(format!("{}: missing duration units", opt).into()),
        _ => Err(format!("{}: invalid duration units {:?}", opt, us).into()),
    }
}

#[cfg(test)]
mod test {
    use crate::parse_duration;
    use rand::{prelude::IteratorRandom, thread_rng, Rng};
    use std::time::Duration;

    #[test]
    fn test_parse_duration_ok() {
        let mut rng = thread_rng();

        let unit_convs: [(Vec<&str>, fn(u64) -> Duration); 5] = [
            (vec!["usec", "us", "µs"], Duration::from_micros),
            (vec!["msec", "ms"], Duration::from_millis),
            (vec!["seconds", "second", "sec", "s"], Duration::from_secs),
            (vec!["minutes", "minute", "min", "m"], |n: u64| {
                Duration::from_secs(n * 60)
            }),
            (vec!["hours", "hour", "hr", "h"], |n: u64| {
                Duration::from_secs(n * 60 * 60)
            }),
        ];

        let pads = ["", " ", "\t"];

        for (unit_strs, conv_func) in unit_convs {
            for unit_str in unit_strs {
                for _ in 0..100 {
                    let u: String = unit_str
                        .chars()
                        .map(|ch| {
                            if rng.gen::<bool>() {
                                ch.to_ascii_uppercase()
                            } else {
                                ch
                            }
                        })
                        .collect();

                    let n = rng.gen_range(0..1000);
                    let exp = conv_func(n);
                    let pad_pfx = pads.into_iter().choose(&mut rng).unwrap();
                    let pad_ifx = pads.into_iter().choose(&mut rng).unwrap();
                    let pad_sfx = pads.into_iter().choose(&mut rng).unwrap();
                    let input = format!("{pad_pfx}{n}{pad_ifx}{u}{pad_sfx}");

                    match parse_duration(input.clone(), "-p") {
                        Ok(d) => assert_eq!(
                            d,
                            conv_func(n),
                            "case failed: {:?}, expected: {:?}, got: {:?}",
                            input,
                            exp,
                            d
                        ),
                        Err(e) => assert!(false, "case failed: {:?}, got err: {:?}", input, e),
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_duration_err() {
        let cases = &[
            ("", "-p: invalid duration"),
            ("x", "-p: invalid duration"),
            ("10", "-p: missing duration units"),
            ("10x", "-p: invalid duration units \"x\""),
        ];

        for (s, err) in cases {
            assert_eq!(
                parse_duration(s.to_string(), "-p").unwrap_err().to_string(),
                err.to_string(),
            );
        }
    }
}
