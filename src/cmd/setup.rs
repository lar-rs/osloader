use async_std::io;
use async_std::path::PathBuf;
use log::info;
// use async_std::fs;
use crate::rpi as os;


// use prettytable::{
    // Table,
    // Row,Cell,
    // table,
    // row,
    // cell,
// };

// use console::{style, Term};
use structopt::StructOpt;
// use crate::can;

///run to setup sd-card on local pc
#[derive(Debug,StructOpt)]
pub struct Opt {
    ///🔧 backup directory
    #[structopt(short = "d", long = "dir",  default_value = "config/lar-pi")]
    dir: String,
    ///🔧 boot dir 
    #[structopt(short = "b", long = "boot",  default_value = "/media/sascha/boot")]
    boot: String,
    // #[structopt(short= "", long = "json")]
    // conf: String,

}



impl Opt {
    pub async fn run(&self) -> io::Result<()> {
        let dir = PathBuf::from(&self.boot);
        info!("run subcommand setup");
        os::setup_config(&dir).await?;
        os::enable_ssh(&dir).await?;
        os::enable_wlan(&dir).await?;
        Ok(())
    }
    
}