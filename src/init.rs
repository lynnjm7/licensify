extern crate git2;

use self::git2::Repository;

use std::fs;
use std::env;
use std::path::PathBuf;


fn create_licensify_dir(path: &PathBuf) {
    println!("Creating '~/.licensify' directory...");
    match fs::create_dir(path) {
        Ok(_) => println!("    licensify directory created!"),
        Err(_) => println!("    unable to create licensify direcctory!"),
    };
}

fn clone_licenses(path: &PathBuf) {
    println!("Clonging license templates...");
    let url = "https://github.com/lynnjm7/licenses.git";

    match Repository::clone(url, path) {
        Ok(_) => println!("    license templates have been cloned!"),
        Err(_) => println!("    we were unable to clone the license templates!"),
    }
}

pub fn setup_licensify() {
    let mut home_dir = env::home_dir().unwrap();

    // Create the home directory (for storing configuration files and the license snippets)
    home_dir.push(".licensify");
    create_licensify_dir(&home_dir);

    // Clone the license templates into the "license" directory within the home directory
    home_dir.push("license");
    clone_licenses(&home_dir);
}
