use structopt::*;
// use clap_flags::Log;
use async_std::io;
use super::rpi as target;

///📢 subcommands 
#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "setup", about = "run to setup sd-card on local pc")]
    ///setup os
    Setup(target::Opt),
    // #[structopt(name = "upgrade", about = "upgrade c")]
    // Upgrade,

}

impl Cmd {
    pub async fn run(&self) -> io::Result<()> {
        match &self {
            Cmd::Setup(opt) => opt.run().await
        }
    }
}
    
///🌐  LAR OS loader 
#[derive(Debug, StructOpt)]
#[structopt(name = "osloader", about = "🌐 LAR OS Loader cli")]
pub struct Args {
    /// Activate debug mode short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,
    // 📝 log level 
    // 
    /// 📢 subcommand to run.
    #[structopt(subcommand)]
    cmd: Cmd,
}

impl Args {
    #[inline]
    pub async fn command(&self) -> io::Result<()> {
        self.cmd.run().await
    }
}
// #[derive(Debug, StructOpt)]
// #[structopt(name = "Socketcan", about = "  🧰 An example of StructOpt usage.")]
// struct Opt {
    // 🔧 Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    // #[structopt(short, long)]
    // debug: bool,

    // interface name
    // we don't want to name it "speed", need to look smart
    // #[structopt(short = "v", long = "velocity", default_value = "42")]
    // speed: f64,
    
//     #[structopt(flatten)]
//     verbosity: clap_flags::Verbosity,
    //  🔧 Output JSON instead of human readable messages
//     #[structopt(name = "json", long = "json")]
//     json: bool,
//       /// The subcommand to run.
//     // pub cmd: Command,
    // ⏱  Message interval in milliseconds
    // #[structopt(name = "interval", long = "interval", default_value = "1000")]
    // interval: u64,
//     /// ⦨  scale signal value
//     #[structopt(name = "scale", long = "scale", default_value = "1.0")]
//     scale: f64,
// }
// 
// impl Cli {
//   /// Initialize a logger.
//   #[inline]
//   pub fn log(&self, name: &str) -> ::Result<()> {
//     self
//       .logger
//       .log(self.verbosity.log_level(), name)
//       .context(::ErrorKind::Log)?;
//     Ok(())
// }

