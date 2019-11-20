use lazy_static::lazy_static;

lazy_static!{
    static ref TEMP: RwLock<BTreeSet<PathBuf>> = RwLock::new(BTreeMap::new());
}

pub struct Confing {
    pub image: String,
    pub conf : String,
    pub git:   String,
}


pub struct Config {
    dev: String,
    rpi:Option<RPiConfig>,
}



impl Default for Config {
    fn default() -> Config {
        Config{
            dev: "/dev/sdc",
        }
    }
}



pub async fn config() -> io::Result<Config> {
    Ok(Config::default())
}


