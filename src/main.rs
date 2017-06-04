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
        println!("list available licenses");
        return;
    }

    let license = cmd_args.value_of("license").unwrap();
    println!("{}", license);

    // Setup command line arguments
    //
    // Process command line arguments
    // if init: run initial setup functions
    //
    // if list: list possible options
    //
    // Read config from file and set values accordingly (not command line args will overwrite this)
    //
    // Given the license to produce, and the values to be used in the template, process the
    // template, producing the final license output.

    println!("Hello, world!");
}
