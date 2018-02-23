extern crate mustache;

use std::env;
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;

use self::mustache::MapBuilder;

struct License {
    name: String,
    organization: String,
    year: String,
    project: String,
}

impl License {
    fn new(name: String) -> License {
        License {
            name: name,
            organization: "".to_string(),
            year: "".to_string(),
            project: "".to_string(),
        }
    }

    fn set_org(mut self, org: String) -> Self {
        self.organization = org;
        self
    }

    fn set_year(mut self, year: String) -> Self {
        self.year = year;
        self
    }

    fn set_project(mut self, project: String) -> Self {
        self.project = project;
        self
    }

    fn render(&self) -> String {
        let mut lic_path = get_template_dir();
        lic_path.push(&self.name);
        lic_path.set_extension("txt");

        let file = match File::open(lic_path) {
            Ok(x) => x,
            Err(_) => {
                println!("Invalid license. Try again...");
                process::exit(-1)
            }
        };

        let mut buffer = BufReader::new(file);
        let mut contents = String::new();
        match buffer.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(_) => {
                println!("Unable to read license file.");
                process::exit(-1);
            }
        }

        let template = mustache::compile_str(contents.as_str()).unwrap();

        let data = MapBuilder::new()
            .insert_str("organization", self.organization.as_str())
            .insert_str("year", self.year.as_str())
            .insert_str("project", self.project.as_str())
            .build();

        template.render_data_to_string(&data).unwrap()
    }
}

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

pub fn fetch_license_text(license: &str, org: &str, year: &str, project: &str) -> String {
    License::new(license.to_string())
        .set_org(org.to_string())
        .set_year(year.to_string())
        .set_project(project.to_string())
        .render()
}
