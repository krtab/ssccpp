mod lib;
use lib::*;

use clap::{App, Arg};
use hostname::get_hostname;
use std::fs::File;
use std::io::{self, BufReader, stdout};


fn main() -> io::Result<()> {
    let hostname = get_hostname().unwrap();
    let cliargs = App::new("SSCCPP, the Simple Switch Case Config PreProcessor")
        .version("0.1.0")
        .author("Arthur Carcano <arthur.carcano@inria.fr>")
        .about("SSCCPP parses a configuration file to adapt it to the current host.")
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .value_name("DELIMITER")
                .takes_value(true)
                .default_value(">>>>>>>>"),
        )
        .arg(
            Arg::with_name("path")
                .required(true)
                .index(1)
                .help("Path of the input file to use."),
        )
        .arg(
            Arg::with_name("ident")
                .short("id")
                .long("ident")
                .value_name("MACHINE_NAME")
                .takes_value(true)
                .default_value(&hostname),
        )
        .get_matches();

    // Opts unwrapping
    let filepath = cliargs.value_of("path").unwrap();
    let delimiter = cliargs.value_of("delimiter").unwrap();
    let ident = cliargs.value_of("ident").unwrap();

    let parser = Parser::new(ident, delimiter).unwrap();


    let file = File::open(filepath)?;
    let bufed_file = BufReader::new(file);
    let mut stdout = stdout();
    parser.process(bufed_file, &mut stdout)
}
