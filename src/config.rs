extern crate chrono;
extern crate toml;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;
use std::process;

use self::chrono::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct ConfigSettings {
    pub default_license: String,
    pub organization: String,
    pub year: String,
    pub project: String,
}

impl ConfigSettings {
    fn new() -> ConfigSettings {
        ConfigSettings {
            default_license: String::new(),
            organization: String::new(),
            year: String::new(),
            project: String::new(),
        }
    }

    fn setup_default_license(mut self) -> Self {
        let mut license = prompt_user("Enter preferred license (default = mit): ");
        if license.is_empty() {
            license = String::from("mit");
        }

        self.default_license = license.to_lowercase();
        self
    }

    fn setup_organization(mut self) -> Self {
        self.organization = prompt_user("Enter preferred organization name: ");
        self
    }

    fn setup_year(mut self) -> Self {
        let current_year = Local::now().year().to_string();
        let mut year = prompt_user(&format!(
            "Enter preferred copyright year (default = {}): ",
            current_year
        ));

        if year.is_empty() {
            year = current_year;
        }

        self.year = year;
        self
    }

    fn setup_project(mut self) -> Self {
        self.project = prompt_user("Enter preferred project name: ");
        self
    }

    fn setup_finalize(self) -> Self {
        println!("\n====================");
        println!("Writing config file to ~/.licensify/config.toml...");

        let config_file_path = get_config_file_path();
        let mut config_file = File::create(config_file_path).unwrap();
        match config_file.write_all(toml::to_string(&self).unwrap().into_bytes().as_slice()) {
            Ok(_) => {}
            Err(_) => {
                println!("Unable to write config.toml file.");
                process::exit(-1);
            }
        };

        self
    }
}

fn prompt_user(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().ok().expect(
        "Unable to flush stdout buffer",
    );

    let mut value = String::new();
    io::stdin().read_line(&mut value).expect(
        "Unable to read value from prompt",
    );

    value.trim().to_string()
}

fn get_config_file_path() -> PathBuf {
    let mut path = env::home_dir().unwrap();
    path.push(".licensify/config.toml");
    path
}

pub fn init_config() {
    println!("\n\n====================");
    println!("Setting up default configuration settings...\n");
    let _config = ConfigSettings::new()
        .setup_default_license()
        .setup_organization()
        .setup_year()
        .setup_project()
        .setup_finalize();
}

pub fn fetch_config() -> ConfigSettings {
    let config_file_path = get_config_file_path();
    let mut config_file = File::open(config_file_path).unwrap();
    let mut contents = String::new();
    config_file.read_to_string(&mut contents).unwrap();
    toml::from_str(contents.as_str()).unwrap()
}
