/// Raspberry pi 
/// 
use log::info;
use async_std::io;
use async_std::path::{PathBuf,Path};
use async_std::fs;
use async_std::task;
// pub struct RasPi {
    // sysroot:
// }

use crate::user;

pub async fn boot_config() -> PathBuf {
    PathBuf::from("config/pican2.txt")
}

pub async fn rootfs() -> io::Result<PathBuf> {
   let path =PathBuf::from("/media/").join(&user::username()).join("rootfs");
   Ok(path)
}


pub async fn setup_config(boot: &Path) -> io::Result<()> {
    info!("rpi: setup config");
    let src   =  boot_config().await;
    let dest  = boot.join("config.txt");
    let _ = fs::copy(src.as_path(),dest.as_path()).await?;
    Ok(())
}

pub async fn enable_ssh(boot:&Path) -> io::Result<()>  {
    info!("rpi : enable ssh");
    let _ = fs::File::create(boot.join("ssh")).await?;
    // let pk = user::public_key();
    // let pissh = rootfs().await?.join(".ssh");
    // fs::create_dir(&pissh).await?;
    // let _ = fs::copy(pk.as_path(),pissh.join("id_rsa.pub").as_path()).await?;

    // let pk = 
    // if let Some(homedir) =  
    Ok(())
}
pub async fn enable_wlan(boot:&Path) -> io::Result<()> {
    info!("rpi : enable wlan");
    let src  =     PathBuf::from("config/larintern.conf");
    let dest =    boot.join("wpa_supplicant.conf");
    let _ = fs::copy(src.as_path(),dest.as_path()).await?;
 
    Ok(())
}
pub async fn setup_firmware(boot:&Path) -> io::Result<()> {
     
    // git clone --depth 1 https://github.com/RPi-Distro/firmware-nonfree.git
    Ok(())
}

pub async fn pigen() {
   // https://github.com/RPi-Distro/pi-gen.git
}