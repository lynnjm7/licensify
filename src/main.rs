extern crate licensify;
extern crate clap;

use std::process;

use clap::{App, Arg};
use licensify::{init, config, license};

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn setup_cmd_args<'a, 'b>() -> clap::App<'a, 'b> {
    App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about("Creates a license file for a project")
        .arg(Arg::with_name("init").short("i").long("init").help(
            "Runs the initial setup command",
        ))
        .arg(Arg::with_name("list").short("l").long("list").help(
            "List the available licenses",
        ))
        .arg(Arg::with_name("organization").short("o").long("organization").value_name("ORG").help(
            "Set the organization value to use in the license. This overrides the config file setting.",
        ))
        .arg(Arg::with_name("year").short("y").long("year").value_name("YEAR").help(
            "Set the year value to use in the license. This overrides the config file setting.",
        ))
        .arg(Arg::with_name("project").short("p").long("project").value_name("PROJECT").help(
            "Set the project value to use in the license. This overrides the config file setting.",
        ))
        .arg(Arg::with_name("license").long("license").index(1))
}

fn main() {
    let cmd_args = setup_cmd_args().get_matches();
    if cmd_args.is_present("init") {
        init::setup_licensify();
        config::init_config();
        println!("Initial configuration finished! Happy licensing!! :-)");
        return;
    }

    if cmd_args.is_present("list") {
        license::list_licenses();
        return;
    }

    let license = match cmd_args.value_of("license") {
        Some(x) => String::from(x),
        None => {
            println!("Please enter a valid license. Use --help for usage information.");
            process::exit(-1);
        }
    };

    // Get the config file contents and merge the provided flag options with the
    // file values. The flag options should override the config file settings.
    let mut config = config::fetch_config();
    config.organization = match cmd_args.value_of("organization") {
        Some(x) => String::from(x),
        None => config.organization,
    };

    config.year = match cmd_args.value_of("year") {
        Some(x) => String::from(x),
        None => config.year,
    };

    config.project = match cmd_args.value_of("project") {
        Some(x) => String::from(x),
        None => config.project,
    };

    // Generate the license text from the given paramaters
    let license_txt =
        license::fetch_license_text(license, config.organization, config.year, config.project);

    // Output the license text to stdout
    println!("{}", license_txt);
}
