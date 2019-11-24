/// Parent system dirs
///  $HOME/.ssh wird ermittelt um ssh key zu ubertragen.

// use log::info;
// use async_std::io;
use async_std::path::{PathBuf};
// use async_std::fs;
// use async_std::task;
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

pub struct User {
    ///🔧 username 
    pub name: String,
}

#[derive(Debug,StructOpt)]
pub struct Opt {
    ///🔧 username 
    #[structopt(short = "n", long = "name",  default_value = "sascha")]
    name: String,
}


// pub struct RasPi {
    // sysroot:
// }


pub async fn generate_keys()  {
    
}