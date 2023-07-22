use std::path::PathBuf;
enum UserOS {
    Linux,
    MacOS,
    Windows,
}

impl UserOS {
    pub fn get() -> Self {
        match std::env::consts::OS {
            "linux" => UserOS::Linux,
            "macos" => UserOS::MacOS,
            "windows" => UserOS::Windows,
            unsupported => {
                panic!("Your Operating System, {unsupported}, is not currently supported... =(")
            }
        }
    }
}

#[derive(Debug)]
pub struct WriteDestination(pub PathBuf);

impl TryFrom<UserOS> for WriteDestination {
    type Error = anyhow::Error;
    fn try_from(value: UserOS) -> Result<Self, anyhow::Error> {
        let destination = match value {
            UserOS::Linux | UserOS::MacOS => PathBuf::from("/tmp/"),
            UserOS::Windows => std::env::temp_dir(),
        };

        Ok(Self(destination))
    }
}

pub fn get_os_tempdir() -> WriteDestination {
    match WriteDestination::try_from(UserOS::get()) {
        Ok(dest) => dest,
        Err(_) => panic!("Failed to query for User's OS type ... "),
    }
}
