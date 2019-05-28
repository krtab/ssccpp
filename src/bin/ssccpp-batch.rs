use ssccpp::Parser;

use clap::{App, Arg};
use hostname::get_hostname;
use std::fs::File;
use std::io::{self, BufReader, stdout};
use walkdir::WalkDir;
use std::path::Path;


fn main() -> io::Result<()> {
    let hostname = get_hostname().unwrap();
    let cliargs = App::new("SSCCPP, the Simple Switch Case Config PreProcessor")
        .version("0.1.0")
        .author("Arthur Carcano <arthur.carcano@inria.fr>")
        .about("ssccpp-batch applies ssccpp to all files in a directory, copying the directory structure into another one.")
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .value_name("DELIMITER")
                .takes_value(true)
                .default_value(">>>>>>>>"),
        )
        .arg(
            Arg::with_name("ident")
                .short("id")
                .long("ident")
                .value_name("MACHINE_NAME")
                .takes_value(true)
                .default_value(&hostname),
        )
        .arg(
            Arg::with_name("from")
            .required(true)
            .help("Path of the input directory."),
        )
        .arg(
            Arg::with_name("into")
            .required(true)
            .help("Path of the output directory.")
        )
        .get_matches();

    // Opts unwrapping
    let fromstr = cliargs.value_of("from").unwrap();
    let intostr = cliargs.value_of("into").unwrap();
    let frompath = Path::new(fromstr);
    let intopath = Path::new(intostr);
    let delimiter = cliargs.value_of("delimiter").unwrap();
    let ident = cliargs.value_of("ident").unwrap();

    let parser = Parser::new(ident, delimiter).unwrap();

    for entry in WalkDir::new(frompath) {
        let entry = entry?;
        let basename = entry.path().strip_prefix(frompath).unwrap();
        println!("{} -> {} ({})",
            entry.path().display(),
            intopath.join(basename).display(),
            if entry.file_type().is_dir() {"D"} else {"F"}
        );
    }
    Ok(())
}
