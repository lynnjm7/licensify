extern crate licensify;
extern crate clap;

use clap::{App, Arg};
use licensify::{init, license};


fn setup_cmd_args<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("Licensify")
        .version("1.0")
        .author("Josh Lynn <lynnjm7@gmail.com")
        .about("Creates a license file for a project")
        .arg(Arg::with_name("init")
                 .short("i")
                 .long("init")
                 .help("Runs the initial setup command"))
        .arg(Arg::with_name("list")
                 .short("l")
                 .long("list")
                 .help("List the available licenses"))
        .arg(Arg::with_name("license").long("license").index(1))
}

fn main() {
    let cmd_args = setup_cmd_args().get_matches();
    if cmd_args.is_present("init") {
        init::setup_licensify();
        return;
    }

    if cmd_args.is_present("list") {
        license::list_licenses();
        return;
    }

    let license = match cmd_args.value_of("license") {
        Some(x) => x,
        None => ""
    };

    // TODO I'm sure there's a better way to handle this error processing... but this works for now
    if license == "" {
        println!("Please enter a valid license. Use --help for usage information.");
        return;
    }

    println!("{}", license);

    // Read config from file and set values accordingly (note command line args will overwrite this)
    //
    // Given the license to produce, and the values to be used in the template, process the
    // template, producing the final license output.

    let license_txt = license::fetch_license_text(license);
    println!("{}", license_txt);
}
