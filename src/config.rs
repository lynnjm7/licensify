extern crate chrono;

use std::io::prelude::*;
use std::io;

use self::chrono::prelude::*;

struct ConfigSettings {
    default_license: String,
    organization: String,
    year: String,
    project: String,
}

impl ConfigSettings {
    fn new() -> ConfigSettings {
        ConfigSettings {
            default_license: "".to_string(),
            organization: "".to_string(),
            year: "".to_string(),
            project: "".to_string(),
        }
    }

    fn setup_default_license(mut self) -> Self {
        let mut license = prompt_user("Enter preferred license (default = mit): ");
        if license.is_empty() {
            license = "mit".to_string();
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
        // TODO(lynnjm7): Save the ConfigSettings to a the ~/.licensify/config.toml file
        println!("\n====================");
        println!("Writing config file to ~/.licensify/config.toml...");


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

pub fn fetch_config() {
    // TODO(lynnjm7): Get and serialize the config file to return the default
    // settings
    println!("fetching config settings...");
}
