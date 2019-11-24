/// Raspberry pi 
/// 
use log::info;
use async_std::io;
use async_std::path::{PathBuf,Path};
use async_std::fs;
use structopt::StructOpt;

// use async_std::task;
// pub struct RasPi {
    // sysroot:
// }

use crate::user;

pub async fn config_txt() -> PathBuf {
    PathBuf::from("config/home-rpi/config.txt")
}

pub async fn wpa() -> PathBuf {
    PathBuf::from("config/home-rpi/wpa_supplicant.conf")
}
pub async fn rootfs() -> io::Result<PathBuf> {
   let path =PathBuf::from("/media/").join(&user::username()).join("rootfs");
   Ok(path)
}

// pub async fn setup_firmware(boot:&Path) -> io::Result<()> {
     
    // git clone --depth 1 https://github.com/RPi-Distro/firmware-nonfree.git
    // Ok(())
// }
// 
pub async fn pigen() {
   // https://github.com/RPi-Distro/pi-gen.git
}

/// raspberry setup sd-card on local pc
#[derive(Debug,StructOpt)]
pub struct Opt {
    ///🔧 backup directory
    #[structopt(short = "d", long = "dir",  default_value = "config/lar-pi")]
    dir: String,
    ///🔧 boot dir 
    #[structopt(short = "b", long = "boot",  default_value = "/media/sascha/boot")]
    boot: PathBuf,
    // #[structopt(short= "", long = "json")]
    // conf: String,
    // Input file
    //  #[structopt(parse(from_os_str))]
    //  input: PathBuf,
}



impl Opt {
    pub async fn run(&self) -> io::Result<()> {
        // let dir = PathBuf::from(&self.dir);
        info!("run subcommand setup");
        self.setup_config().await?;
        self.enable_ssh().await?;
        self.enable_wlan().await?;
        Ok(())
    }

    pub async fn setup_config(&self) -> io::Result<()> {
        info!("rpi: setup config");
        let src   =  config_txt().await;
        let dest  =  self.boot.join("config.txt");
        println!("boot{}",self.boot.to_str().unwrap());
        let _ = fs::copy(src.as_path(),dest.as_path()).await?;
        Ok(())
    }
    
    pub async fn enable_ssh(&self) -> io::Result<()>  {
        info!("rpi : enable ssh");
        let _ = fs::File::create(self.boot.join("ssh")).await?;
        // let pk = user::public_key();
        // let pissh = rootfs().await?.join(".ssh");
        // fs::create_dir(&pissh).await?;
        // let _ = fs::copy(pk.as_path(),pissh.join("id_rsa.pub").as_path()).await?;
    
        // let pk = 
        // if let Some(homedir) =  
        Ok(())
    }
    
    pub async fn enable_wlan(&self) -> io::Result<()> {
        info!("rpi : enable wlan");
        let src  =    wpa().await;
        let dest =    self.boot.join("wpa_supplicant.conf");
        let _ = fs::copy(src.as_path(),dest.as_path()).await?;
        Ok(())
    }
    
}