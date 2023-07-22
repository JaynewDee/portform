use std::path::PathBuf;

enum UserOS {
    Linux,
    MacOS,
    Windows,
    Unsupported,
}

impl UserOS {
    pub fn get() -> Self {
        // exhaustive match patterns
        match std::env::consts::OS {
            "linux" => UserOS::Linux,
            "macos" => UserOS::MacOS,
            "windows" => UserOS::Windows,
            _ => UserOS::Unsupported, // wildcard
        }
    }
}

#[derive(Debug)]
pub struct TempPath(pub PathBuf);

const UNIX_PATH: &str = "/tmp/portform_config.json";

// TryFrom trait enables idiomatic conversion between custom types
impl TryFrom<UserOS> for TempPath {
    type Error = anyhow::Error;

    fn try_from(value: UserOS) -> Result<Self, anyhow::Error> {
        // exhaust all possible enum variants
        let destination = match value {
            UserOS::Linux | UserOS::MacOS => PathBuf::from(UNIX_PATH),
            UserOS::Windows => std::env::temp_dir().join("portform_config.json"),
            UserOS::Unsupported => panic!("Your OS is not currently supported ... =("),
        };

        let temp_path = Self(destination);

        Ok(temp_path)
    }
}

pub fn get_os_config_path() -> Result<TempPath, anyhow::Error> {
    // Attempt conversion, propagate any errors to be handled by caller
    TempPath::try_from(UserOS::get())
}
