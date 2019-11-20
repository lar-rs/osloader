use setde::Deserialize;
use lazy_static::lazy_static;

lazy_static!{
    static ref TEMP: RwLock<BTreeSet<PathBuf>> = RwLock::new(BTreeMap::new());
}

type Commands = BTreeMap<String, String>;

pub struct Confing {
    pub image: String,
    pub conf : String,
    pub git:   String,
}

#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
/// Configuration file
pub struct ConfigFile {
    pre_commands: Option<Commands>,
    commands: Option<Commands>,
    git_repos: Option<Vec<String>>,
    // disable: Option<Vec<Step>>,
    remote_topgrades: Option<Vec<String>>,
    ssh_arguments: Option<String>,
    git_arguments: Option<String>,
    tmux_arguments: Option<String>,
    set_title: Option<bool>,
    assume_yes: Option<bool>,
    yay_arguments: Option<String>,
    no_retry: Option<bool>,
    run_in_tmux: Option<bool>,
    cleanup: Option<bool>,
    // only: Option<Vec<Step>>,
}

impl ConfigFile {
    // fn ensure(base_dirs: &BaseDirs) -> Result<PathBuf, Error> {
    //     let config_path = base_dirs.config_dir().join("topgrade.toml");
    //     if !config_path.exists() {
    //         write(&config_path, include_str!("../config.example.toml"))
    //             .map_err(|e| {
    //                 debug!(
    //                     "Unable to write the example configuration file to {}: {}. Using blank config.",
    //                     config_path.display(),
    //                     e
    //                 );
    //                 e
    //             })
    //             .context(ErrorKind::Configuration)?;
    //         debug!("No configuration exists");
    //     }

    //     Ok(config_path)
    // }

    // /// Read the configuration file.
    // ///
    // /// If the configuration file does not exist the function returns the default ConfigFile.
    // fn read(base_dirs: &BaseDirs) -> Result<ConfigFile, Error> {
    //     let config_path = Self::ensure(base_dirs)?;
    //     let mut result: Self = toml::from_str(&fs::read_to_string(config_path).context(ErrorKind::Configuration)?)
    //         .context(ErrorKind::Configuration)?;

    //     if let Some(ref mut paths) = &mut result.git_repos {
    //         for path in paths.iter_mut() {
    //             *path = shellexpand::tilde::<&str>(&path.as_ref()).into_owned();
    //         }
    //     }

    //     debug!("Loaded configuration: {:?}", result);

    //     Ok(result)
    // }

    // fn edit(base_dirs: &BaseDirs) -> Result<(), Error> {
    //     let config_path = Self::ensure(base_dirs)?;
    //     let editor = editor();

    //     debug!("Editing {} with {}", config_path.display(), editor);
    //     Command::new(editor)
    //         .arg(config_path)
    //         .spawn()
    //         .and_then(|mut p| p.wait())
    //         .context(ErrorKind::Configuration)?;
    //     Ok(())
    // }
}

pub struct Config {
    dev: String,
    rpi:Option<RPiConfig>,
    cfgfile: ConfigFile,
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

