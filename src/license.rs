use std::env;
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;

fn get_template_dir() -> PathBuf {
    let mut lic_dir = env::home_dir().unwrap();
    lic_dir.push(".licensify/license/templates");
    lic_dir
}

pub fn list_licenses() {
    // in the '~/.licensify/licenses/templates' directory the '.txt' files are the license 
    // templates. The names of these files are possible license options that can be used to
    // generate a licenese.
    let lic_dir = get_template_dir();

    let licenses = match fs::read_dir(lic_dir) {
        Ok(x) => x,
        Err(_) => {
            println!("Please initialize with --init.");
            process::exit(-1);
        }
    };

    for lic in licenses {
        let path = lic.unwrap().path();
        let lic_name = path.file_stem().unwrap().to_str().unwrap();

        println!("{}", lic_name);
    }
}

pub fn fetch_license_text(license: &str) -> String {
    let mut lic_path = get_template_dir(); 
    lic_path.push(license);
    lic_path.set_extension("txt");

    let file = match File::open(lic_path) {
        Ok(x) => x,
        Err(_) => {
            println!("Invalid license. Try again...");
            process::exit(-1)
        }  
    };

    // TODO(lynnjm7): Change this to use an impl on a struct notion...
    let mut buffer = BufReader::new(file);
    let mut contents = String::new();
    buffer.read_to_string(&mut contents);

   return contents;
}
