//! convert verbosty level from count of -v flags into enum from a logger crate of your choice

use bpaf::*;

// generally you'd use this from the log crate itself
#[derive(Debug, Copy, Clone)]
pub enum LevelFilter {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Bpaf)]
#[bpaf(options)]
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Options {
    #[bpaf(external)]
    verbose: LevelFilter,
    /// number of potatoes
    #[bpaf(fallback(3))]
    potato: usize,
}

fn verbose() -> impl Parser<LevelFilter> {
    short('v')
        .help("Verbosity level, use multiple times for more verbosity")
        .req_flag(())
        .count()
        .map(|l| {
        .map(|l| match l {
            0 => LevelFilter::Off,
            1 => LevelFilter::Error,
            2 => LevelFilter::Warn,
            3 => LevelFilter::Info,
            4 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        })
}

fn main() {
    println!("{:#?}", options().run());
}
