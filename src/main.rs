use std::error::Error;

use clap::Parser;
use duration_str::parse as parse_duration;
use tokio::{
    io::{stdin, stdout, AsyncBufReadExt, AsyncWriteExt, BufReader},
    time::{interval, MissedTickBehavior},
};

#[derive(Parser, Debug)]
struct Opts {
    #[clap(
        short,
        long,
        default_value = "1.0",
        help = "Output rate in lines per period (-p)"
    )]
    rate: f32,

    #[clap(
        short,
        long,
        default_value = "1s",
        help = "Time period to apply output rate to"
    )]
    period: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let mut input = BufReader::new(stdin());
    let mut tick = interval(parse_duration(&opts.period)?.div_f32(opts.rate));
    tick.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        let mut buf = String::new();

        tokio::select! {
            biased;
            _ = tick.tick() => {
                input.read_line(&mut buf).await?;
                stdout().write_all(buf.as_bytes()).await?;
            }
            _ = input.read_line(&mut buf) => {}
        }
    }
}
