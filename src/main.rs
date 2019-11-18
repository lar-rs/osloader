
use osloader::cli::Args;
use async_log::span;
use log::info;
use async_std::io;

fn setup_logger() {
    let logger = femme::pretty::Logger::new();
    async_log::Logger::wrap(logger, || 12)
        .start(log::LevelFilter::Trace)
        .unwrap();
}

#[paw::main]
fn main(args: Args) -> io::Result<()> {
    setup_logger();
    span!("new level, depth={}", 1, {
        let x = "beep";
        info!("look at this value, x={}", x);

        span!("new level, depth={}", 2, {
            let y = "boop";
            info!("another nice value, y={}", y);
        })
    });
    Ok(())
}
