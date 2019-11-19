/// Raspberry pi 
/// 
use log::info;
use async_std::io;
use async_std::path::{PathBuf,Path};
use async_std::fs;


pub async fn get_config() -> PathBuf {
    PathBuf::from("config/pican2.txt")
}


pub async fn setup_config(boot: &Path) -> io::Result<()> {
    info!("rpi: setup config");
    let src   =  get_config().await;
    let dest  = boot.join("config.txt");
    let _ = fs::copy(src.as_path(),dest.as_path()).await?;
    Ok(())
}

pub async fn enable_ssh(boot:&Path) -> io::Result<()>  {
    info!("rpi : enable ssh");
    let _ = fs::File::create(boot.join("ssh")).await?;
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