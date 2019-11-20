
use osloader::cli::Args;
use async_log::span;
use log::info;
use async_std::io;
use async_std::task;

fn setup_logger() {
    let logger = femme::pretty::Logger::new();
    async_log::Logger::wrap(logger, || 12)
        .start(log::LevelFilter::Trace)
        .unwrap();
}

#[paw::main]
fn main(args: Args) -> io::Result<()> {
    setup_logger();
    task::block_on(args.command())
}
