use std::env;
use std::fs;

pub fn list_licenses() {
    // in the '~/.licensify/licenses/templates' directory the '.txt' files are the license 
    // templates. The names of these files are possible license options that can be used to
    // generate a licenese.
    let mut lic_dir = env::home_dir().unwrap();
    lic_dir.push(".licensify");
    lic_dir.push("license");
    lic_dir.push("templates");

    let licenses = fs::read_dir(lic_dir).unwrap();

    for lic in licenses {
        let path = lic.unwrap().path();
        let lic_name = path.file_stem().unwrap().to_str().unwrap();

        println!("{}", lic_name);
    }
}
