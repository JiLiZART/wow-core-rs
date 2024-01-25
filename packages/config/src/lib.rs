use std::{env, path::Path};

use config::Config;

// type ConfigMap<T> = HashMap<String, T>;

pub struct ConfigBuilder {
    pub config: Config,
}

impl ConfigBuilder {
    pub fn new(name: &str, env_name: &str) -> ConfigBuilder {
        let exe = env::current_exe().unwrap();
        let dir = exe.parent().expect("Executable must be in some directory");
        let dir_name = dir.to_str().unwrap();
        let file_name = Path::new(dir_name).join(name.to_owned() + ".toml").display().to_string();

        let settings = Config::builder()
        // Add in `./name.toml`

        .add_source(config::File::with_name(file_name.as_str()).required(false))
        // Add in settings from the environment (with a prefix of env_name)
        .add_source(config::Environment::with_prefix(env_name))
        .build()
        .unwrap();

        ConfigBuilder {
            config: settings,
        }
    }

    pub fn get<'a, T: serde::Deserialize<'a>>(&self, key: &str) -> T {
        self.config.get::<T>(key).unwrap()
    }

    pub fn get_or_default<'a, T: serde::Deserialize<'a>>(&self, key: &str, default: T) -> T {
        self.config.get::<T>(key).unwrap_or(default)
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
