/// Parent system dirs
///  $HOME/.ssh wird ermittelt um ssh key zu ubertragen.

use log::info;
use async_std::io;
use async_std::path::{PathBuf,Path};
use async_std::fs;
use async_std::task;
use std::env;
use users;

pub fn public_key() -> PathBuf {
    let home:PathBuf = env::home_dir().unwrap().into();
    home.join(".ssh/id_rsa.pub")
}

pub fn username() -> String {
    match users::get_current_username() {
        Some(uname) => uname.into_string().unwrap(),
        None        => "root".to_string(),
    }
}

// pub struct RasPi {
    // sysroot:
// }


pub async fn generate_keys()  {
    
}